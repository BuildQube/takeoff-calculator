use crate::state::TakeoffStateHandler;
use crate::utils::lock_mutex;
use napi_derive::napi;
use std::sync::{Arc, Mutex};
use takeoff_core::contour::{
  ContourInput, ContourLineInput, ContourPointOfInterestInput, SurfaceMesh,
};
use takeoff_core::coords::{Point, Point3D};
use takeoff_core::error::TakeoffResult;
use takeoff_core::scale::Scale;
use takeoff_core::unit::{Unit, UnitValue};
use takeoff_core::volume::{ReferenceSurface, ReferenceSurfaceInput, VolumetricResult};

// --- NAPI Input Types (JS-facing) ---

#[napi(object)]
#[derive(Debug, Clone)]
pub struct ContourLineInputJs {
  pub elevation: f64,
  pub unit: Unit,
  pub points: Vec<Point>,
}

#[napi(object)]
#[derive(Debug, Clone)]
pub struct ContourPointOfInterestInputJs {
  pub elevation: f64,
  pub unit: Unit,
  pub point: Point,
}

#[napi(object)]
#[derive(Debug, Clone)]
pub struct ContourInputJs {
  pub id: String,
  pub name: Option<String>,
  pub page_id: String,
  pub lines: Vec<ContourLineInputJs>,
  pub points_of_interest: Vec<ContourPointOfInterestInputJs>,
}

impl From<ContourLineInputJs> for ContourLineInput {
  fn from(js: ContourLineInputJs) -> Self {
    Self {
      elevation: js.elevation,
      unit: js.unit,
      points: js.points,
    }
  }
}

impl From<ContourPointOfInterestInputJs> for ContourPointOfInterestInput {
  fn from(js: ContourPointOfInterestInputJs) -> Self {
    Self {
      elevation: js.elevation,
      unit: js.unit,
      point: js.point,
    }
  }
}

impl From<ContourInputJs> for ContourInput {
  fn from(js: ContourInputJs) -> Self {
    Self {
      id: js.id,
      name: js.name,
      page_id: js.page_id,
      lines: js.lines.into_iter().map(|l| l.into()).collect(),
      points_of_interest: js
        .points_of_interest
        .into_iter()
        .map(|p| p.into())
        .collect(),
    }
  }
}

// --- Volume Result with Units ---

#[napi]
#[derive(Debug, Clone)]
pub struct VolumetricUnitResult {
  cut: UnitValue,
  fill: UnitValue,
  uncovered_area: UnitValue,
}

#[napi]
impl VolumetricUnitResult {
  #[napi(getter)]
  pub fn cut(&self) -> UnitValue {
    self.cut.clone()
  }

  #[napi(getter)]
  pub fn fill(&self) -> UnitValue {
    self.fill.clone()
  }

  #[napi(getter)]
  pub fn uncovered_area(&self) -> UnitValue {
    self.uncovered_area.clone()
  }
}

// --- ContourWrapper ---

#[napi]
#[derive(Debug, Clone)]
pub struct ContourWrapper {
  contour: Arc<Mutex<ContourInput>>,
  scale: Arc<Mutex<Option<Scale>>>,
  surface_mesh: Arc<Mutex<Option<SurfaceMesh>>>,
  state: Arc<TakeoffStateHandler>,
}

#[napi]
impl ContourWrapper {
  pub fn from_input(contour: ContourInput, state: Arc<TakeoffStateHandler>) -> Self {
    Self {
      contour: Arc::new(Mutex::new(contour)),
      scale: Arc::new(Mutex::new(None)),
      surface_mesh: Arc::new(Mutex::new(None)),
      state,
    }
  }

  #[napi(constructor)]
  pub fn new(contour: ContourInputJs) -> Self {
    let input: ContourInput = contour.into();
    Self::from_input(input, Arc::new(TakeoffStateHandler::default()))
  }

  pub fn set_contour(&self, contour: ContourInput) {
    *lock_mutex(self.contour.lock(), "contour")
      .expect("BUG: contour mutex should not be poisoned") = contour;
    let _ = self.rebuild_surface_mesh();
  }

  pub fn set_scale(&self, scale: Scale) {
    *lock_mutex(self.scale.lock(), "scale").expect("BUG: scale mutex should not be poisoned") =
      Some(scale);
    let _ = self.rebuild_surface_mesh();
  }

  pub fn calculate_scale(&self) -> Option<Scale> {
    let mut current_scale: Option<Scale> = None;
    let contour = lock_mutex(self.contour.lock(), "contour").ok()?;
    let bounding_box = contour.bounding_box()?;
    let page_id = contour.page_id.clone();
    drop(contour);

    let geometry = {
      let ((min_x, min_y), (max_x, max_y)) = bounding_box;
      use geo::{Coord, Rect};
      geo::Geometry::Rect(Rect::new(
        Coord { x: min_x, y: min_y },
        Coord { x: max_x, y: max_y },
      ))
    };

    for scale in self.state.get_page_scales(&page_id) {
      if matches!(scale, Scale::Area { .. }) {
        if scale.is_in_bounding_box(&geometry) {
          self.set_scale(scale.clone());
          return Some(scale);
        }
      } else {
        current_scale = Some(scale.clone());
      }
    }
    if let Some(scale) = current_scale {
      self.set_scale(scale.clone());
      return Some(scale);
    }
    None
  }

  fn rebuild_surface_mesh(&self) -> TakeoffResult<()> {
    let scale_guard = lock_mutex(self.scale.lock(), "scale")?;
    if let Some(scale) = scale_guard.as_ref() {
      let contour = lock_mutex(self.contour.lock(), "contour")?;
      match contour.to_surface_mesh(scale) {
        Ok(mesh) => {
          drop(contour);
          drop(scale_guard);
          *lock_mutex(self.surface_mesh.lock(), "surface_mesh")? = Some(mesh);
        }
        Err(_) => {
          drop(contour);
          drop(scale_guard);
          *lock_mutex(self.surface_mesh.lock(), "surface_mesh")? = None;
        }
      }
    } else {
      drop(scale_guard);
      *lock_mutex(self.surface_mesh.lock(), "surface_mesh")? = None;
    }
    Ok(())
  }

  #[napi(getter)]
  pub fn id(&self) -> String {
    lock_mutex(self.contour.lock(), "contour")
      .expect("BUG: contour mutex should not be poisoned")
      .id
      .clone()
  }

  #[napi(getter)]
  pub fn page_id(&self) -> String {
    lock_mutex(self.contour.lock(), "contour")
      .expect("BUG: contour mutex should not be poisoned")
      .page_id
      .clone()
  }

  #[napi(getter)]
  pub fn get_scale(&self) -> Option<Scale> {
    lock_mutex(self.scale.lock(), "scale")
      .ok()
      .and_then(|s| s.clone())
  }

  #[napi]
  pub fn get_surface_points(&self) -> Option<Vec<Point3D>> {
    let mesh_guard = lock_mutex(self.surface_mesh.lock(), "surface_mesh").ok()?;
    mesh_guard.as_ref().map(|mesh| mesh.vertices.clone())
  }

  #[napi]
  pub fn get_z_at(&self, x: f64, y: f64) -> Option<f64> {
    let mesh_guard = lock_mutex(self.surface_mesh.lock(), "surface_mesh").ok()?;
    let mesh = mesh_guard.as_ref()?;
    mesh.z_at(x, y)
  }

  #[napi]
  pub fn get_scatter_data(&self, step: i32) -> Option<Vec<Point3D>> {
    if step <= 0 {
      return None;
    }
    let step = step as usize;

    let contour = lock_mutex(self.contour.lock(), "contour").ok()?;
    let bounding_box = contour.bounding_box()?;
    drop(contour);

    let mesh_guard = lock_mutex(self.surface_mesh.lock(), "surface_mesh").ok()?;
    let surface_mesh = mesh_guard.as_ref()?;

    let (min_x, min_y) = bounding_box.0;
    let (max_x, max_y) = bounding_box.1;
    let mut data: Vec<Point3D> = Vec::new();

    let x_start = min_x.floor() as i32;
    let x_end = max_x.ceil() as i32;
    let y_start = min_y.floor() as i32;
    let y_end = max_y.ceil() as i32;

    for x in (x_start..=x_end).step_by(step) {
      for y in (y_start..=y_end).step_by(step) {
        if let Some(z) = surface_mesh.z_at(x as f64, y as f64) {
          data.push(Point3D::new(x as f64, y as f64, z));
        }
      }
    }
    Some(data)
  }

  /// Compute raw cut/fill volume (pixel-space values) against a reference surface.
  #[napi]
  pub fn raw_volume_against(
    &self,
    reference: ReferenceSurfaceInput,
    cell_size: Option<f64>,
  ) -> Option<VolumetricResult> {
    let mesh_guard = lock_mutex(self.surface_mesh.lock(), "surface_mesh").ok()?;
    let mesh = mesh_guard.as_ref()?;
    let reference = ReferenceSurface::from(reference);
    Some(mesh.volume_against(&reference, cell_size))
  }

  /// Compute unit-aware cut/fill volume against a reference surface.
  /// Returns None if surface mesh or scale is not available.
  #[napi]
  pub fn volume_against(
    &self,
    reference: ReferenceSurfaceInput,
    cell_size: Option<f64>,
  ) -> Option<VolumetricUnitResult> {
    let mesh_guard = lock_mutex(self.surface_mesh.lock(), "surface_mesh").ok()?;
    let mesh = mesh_guard.as_ref()?;
    let scale_guard = lock_mutex(self.scale.lock(), "scale").ok()?;
    let scale = scale_guard.as_ref()?;

    let reference_surface = ReferenceSurface::from(reference);
    let raw = mesh.volume_against(&reference_surface, cell_size);

    let ratio = scale.ratio().ok()?;
    let unit = scale.get_unit();

    // Raw volume is in cubic pixels. Convert: real_volume = raw_volume / ratio^3
    let ratio_cubed = ratio * ratio * ratio;
    let cut_real = raw.cut / ratio_cubed;
    let fill_real = raw.fill / ratio_cubed;
    let uncovered_area_real = raw.uncovered_area / (ratio * ratio);

    Some(VolumetricUnitResult {
      cut: UnitValue::from_volume(unit.get_volume_unit(cut_real as f32)),
      fill: UnitValue::from_volume(unit.get_volume_unit(fill_real as f32)),
      uncovered_area: UnitValue::from_area(unit.get_area_unit(uncovered_area_real as f32)),
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use takeoff_core::contour::ContourLineInput;
  use takeoff_core::coords::Point;
  use takeoff_core::scale::{Scale, ScaleDefinition};
  use takeoff_core::unit::Unit;

  fn test_contour_input() -> ContourInput {
    ContourInput {
      id: "c1".to_string(),
      name: None,
      page_id: "p1".to_string(),
      lines: vec![ContourLineInput {
        elevation: 10.0,
        unit: Unit::Feet,
        points: vec![
          Point::new(0.0, 0.0),
          Point::new(100.0, 0.0),
          Point::new(100.0, 100.0),
          Point::new(0.0, 100.0),
        ],
      }],
      points_of_interest: vec![],
    }
  }

  fn test_scale() -> Scale {
    Scale::Default {
      id: "s1".to_string(),
      page_id: "p1".to_string(),
      scale: ScaleDefinition {
        pixel_distance: 1.0,
        real_distance: 1.0,
        unit: Unit::Feet,
      },
    }
  }

  #[test]
  fn test_contour_wrapper_no_scale() {
    let wrapper = ContourWrapper::from_input(
      test_contour_input(),
      Arc::new(TakeoffStateHandler::default()),
    );
    assert!(wrapper.get_surface_points().is_none());
    assert!(wrapper.get_scatter_data(10).is_none());
    assert!(wrapper.get_z_at(50.0, 50.0).is_none());
  }

  #[test]
  fn test_contour_wrapper_with_scale() {
    let wrapper = ContourWrapper::from_input(
      test_contour_input(),
      Arc::new(TakeoffStateHandler::default()),
    );
    wrapper.set_scale(test_scale());
    let points = wrapper.get_surface_points();
    assert!(points.is_some());
    let points = points.unwrap();
    assert_eq!(points.len(), 4);
    // With 1:1 scale, elevation 10.0 ft = 10.0 px
    assert!((points[0].z - 10.0).abs() < 1e-6);
  }

  #[test]
  fn test_contour_wrapper_scatter_data_with_scale() {
    let wrapper = ContourWrapper::from_input(
      test_contour_input(),
      Arc::new(TakeoffStateHandler::default()),
    );
    wrapper.set_scale(test_scale());
    let scatter = wrapper.get_scatter_data(10);
    assert!(scatter.is_some());
    assert!(!scatter.unwrap().is_empty());
  }
}
