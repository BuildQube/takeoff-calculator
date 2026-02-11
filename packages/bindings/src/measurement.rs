use std::sync::{Arc, Mutex};

use napi_derive::napi;
use takeoff_core::error::TakeoffResult;
use takeoff_core::scale::Scale;
use takeoff_core::unit::UnitValue;
use takeoff_core::{measurement::Measurement, unit::Unit};
use uom::si::f32::{Area, Length};

use crate::state::TakeoffStateHandler;

use napi::Result;
#[napi]
#[derive(Debug, Clone)]
pub struct MeasurementWrapper {
  measurement: Arc<Mutex<Measurement>>,

  scale: Arc<Mutex<Option<Scale>>>,
  area: Arc<Mutex<Option<Area>>>,
  length: Arc<Mutex<Option<Length>>>,
  points: f64,

  // #[serde(skip)]
  state: Arc<TakeoffStateHandler>,
}

#[napi]
impl MeasurementWrapper {
  pub fn new(measurement: Measurement, state: Arc<TakeoffStateHandler>) -> Self {
    let points = match measurement.clone() {
      Measurement::Count { .. } => 1,
      Measurement::Polygon { points, .. } => points.len(),
      Measurement::Polyline { points, .. } => points.len(),
      Measurement::Rectangle { .. } => 4,
    };
    Self {
      measurement: Arc::new(Mutex::new(measurement)),
      scale: Arc::new(Mutex::new(None)),
      area: Arc::new(Mutex::new(None)),
      length: Arc::new(Mutex::new(None)),
      points: points as f64,
      state,
    }
  }

  pub fn set_measurement(&self, measurement: Measurement) {
    *self.measurement.lock().unwrap() = measurement;
    self.recompute_measurements();
  }

  #[napi(getter)]
  pub fn get_points(&self) -> f64 {
    self.points
  }

  pub fn get_count(&self) -> f64 {
    1.0
  }

  fn calculate_area(&self) -> TakeoffResult<Option<Area>> {
    if let Some(scale) = self.scale.lock().unwrap().as_ref() {
      let scale_ratio = scale.ratio()?;

      let raw_area = self.raw_area()?;

      let area = raw_area / (scale_ratio * scale_ratio);
      let res = scale.get_unit().get_area_unit(area as f32);
      return Ok(Some(res));
    }
    Ok(None)
  }

  #[napi(getter)]
  pub fn get_measurement(&self) -> Measurement {
    self.measurement.lock().unwrap().clone()
  }

  #[napi(getter)]
  pub fn get_area(&self) -> Option<UnitValue> {
    if let Ok(Some(area)) = self.get_area_value() {
      return Some(UnitValue::from_area(area));
    }
    None
  }

  pub fn get_area_value(&self) -> TakeoffResult<Option<Area>> {
    let mut area = self.area.lock().unwrap();
    if area.is_none() {
      *area = self.calculate_area()?;
      Ok(*area)
    } else {
      Ok(*area)
    }
  }

  pub fn calculate_scale(&self) -> Option<Scale> {
    let mut current_scale: Option<Scale> = None;
    let measurement = self.measurement.lock().unwrap();
    let geometry = match measurement.to_geometry() {
      Ok(geom) => geom,
      Err(_) => return None, // Invalid geometry, cannot determine scale
    };
    drop(measurement);

    for scale in self.state.get_page_scales(&self.page_id()) {
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

  #[napi]
  pub fn convert_area(&self, unit: Unit) -> Result<Option<f32>> {
    let area = self.calculate_area()?;
    Ok(area.map(|area| unit.convert_area_to_unit(area)))
  }

  pub fn get_length_value(&self) -> TakeoffResult<Option<Length>> {
    let mut length = self.length.lock().unwrap();
    if length.is_none() {
      *length = self.calculate_length()?;
    }
    Ok(*length)
  }

  fn calculate_length(&self) -> TakeoffResult<Option<Length>> {
    if let Some(scale) = self.scale.lock().unwrap().as_ref() {
      let scale_ratio = scale.ratio()?;

      let raw_perimeter = self.raw_perimeter()?;

      let length = raw_perimeter / scale_ratio;
      let res = scale.get_unit().get_unit(length as f32);
      return Ok(Some(res));
    }
    Ok(None)
  }

  #[napi]
  pub fn convert_length(&self, unit: Unit) -> Result<Option<f32>> {
    if let Some(length) = self.calculate_length()? {
      return Ok(Some(unit.convert_length_to_unit(length)));
    }
    Ok(None)
  }

  #[napi(getter)]
  pub fn get_length(&self) -> Result<Option<UnitValue>> {
    if let Some(length) = self.calculate_length()? {
      return Ok(Some(UnitValue::from_length(length)));
    }
    Ok(None)
  }

  pub fn recompute_measurements(&self) -> TakeoffResult<()> {
    let area = self.calculate_area();
    *self.area.lock().unwrap() = area?;

    let length = self.calculate_length();
    *self.length.lock().unwrap() = length?;

    let _ = self.state.compute_group(&self.get_group_id());
    Ok(())
  }

  pub fn set_scale(&self, scale: Scale) {
    *self.scale.lock().unwrap() = Some(scale);
    self.recompute_measurements();
  }

  #[napi(getter)]
  pub fn get_scale(&self) -> Option<Scale> {
    self.scale.lock().unwrap().clone()
  }

  #[napi(getter)]
  pub fn id(&self) -> String {
    self.measurement.lock().unwrap().id().to_string()
  }

  #[napi(getter)]
  pub fn page_id(&self) -> String {
    self.measurement.lock().unwrap().page_id().to_string()
  }

  #[napi(getter)]
  pub fn get_group_id(&self) -> String {
    self.measurement.lock().unwrap().group_id().to_string()
  }

  #[napi(getter)]
  pub fn raw_area(&self) -> Result<f64> {
    let area = self.measurement.lock().unwrap().pixel_area()?;
    Ok(area)
  }

  #[napi(getter)]
  pub fn raw_perimeter(&self) -> Result<f64> {
    let perimeter = self.measurement.lock().unwrap().pixel_perimeter()?;
    Ok(perimeter)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use takeoff_core::{coords::Point, scale::ScaleDefinition, unit::Unit};
  use uom::si::area::square_meter;

  #[test]
  fn test_calculate_area() {
    let measurement = Measurement::Rectangle {
      id: "1".to_string(),
      page_id: "1".to_string(),
      group_id: "1".to_string(),
      points: (Point::new(0.0, 0.0), Point::new(100.0, 50.0)),
    };

    assert_eq!(measurement.pixel_area().unwrap(), 5000.0);
    let measurement_wrapper =
      MeasurementWrapper::new(measurement, Arc::new(TakeoffStateHandler::default()));
    measurement_wrapper.set_scale(Scale::Default {
      id: "1".to_string(),
      page_id: "1".to_string(),
      scale: ScaleDefinition {
        pixel_distance: 100.0,
        real_distance: 2.0,
        unit: Unit::Meters,
      },
    });
    let area = measurement_wrapper.calculate_area().unwrap().unwrap();

    assert_eq!(area.get::<square_meter>(), 2.0);
    assert_eq!(
      measurement_wrapper
        .convert_area(Unit::Meters)
        .unwrap()
        .unwrap(),
      2.0
    );
    assert_eq!(
      measurement_wrapper
        .convert_length(Unit::Meters)
        .unwrap()
        .unwrap(),
      6.0
    );
  }

  #[test]
  fn test_calculate_without_scale() {
    let measurement = Measurement::Rectangle {
      id: "1".to_string(),
      page_id: "1".to_string(),
      group_id: "1".to_string(),
      points: (Point::new(0.0, 0.0), Point::new(100.0, 50.0)),
    };
    let measurement_wrapper =
      MeasurementWrapper::new(measurement, Arc::new(TakeoffStateHandler::default()));
    assert_eq!(measurement_wrapper.raw_area().ok(), Some(5000.0));
    assert_eq!(measurement_wrapper.raw_perimeter().ok(), Some(300.0));
    assert_eq!(
      measurement_wrapper.convert_area(Unit::Meters).unwrap(),
      None
    );
    assert_eq!(
      measurement_wrapper.convert_length(Unit::Meters).unwrap(),
      None
    );
  }

  #[test]
  fn test_pixel_perimeter_polyline() {
    let measurement = Measurement::Polyline {
      id: "1".to_string(),
      page_id: "1".to_string(),
      group_id: "1".to_string(),
      points: vec![Point::new(0.0, 0.0), Point::new(0.0, 1.0)],
    };
    let measurement_wrapper =
      MeasurementWrapper::new(measurement, Arc::new(TakeoffStateHandler::default()));

    assert_eq!(measurement_wrapper.raw_perimeter().ok(), Some(1.0));
    assert_eq!(
      measurement_wrapper.convert_length(Unit::Meters).unwrap(),
      None
    );
  }
}
