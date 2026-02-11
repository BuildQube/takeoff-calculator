use crate::coords::Point;
use crate::error::TakeoffError;
use crate::unit::Unit;
use geo::Contains;
use geo::{Coord, Geometry, Polygon as GeoPolygon, Rect};
use napi_derive::napi;
use serde::{Deserialize, Serialize};

#[napi(object)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ScaleDefinition {
  pub pixel_distance: f64,
  pub real_distance: f64,
  pub unit: Unit,
}

impl ScaleDefinition {
  /// Validate that the scale definition has valid values.
  ///
  /// Returns an error if:
  /// - `pixel_distance` is zero or negative
  /// - `real_distance` is zero or negative
  pub fn validate(&self) -> Result<(), TakeoffError> {
    if self.pixel_distance <= 0.0 {
      return Err(TakeoffError::invalid_scale(format!(
        "pixel_distance must be positive, got {}",
        self.pixel_distance
      )));
    }
    if self.real_distance <= 0.0 {
      return Err(TakeoffError::invalid_scale(format!(
        "real_distance must be positive, got {}",
        self.real_distance
      )));
    }
    Ok(())
  }

  /// Calculate the scale ratio (pixel_distance / real_distance).
  ///
  /// Returns an error if the scale is invalid (zero or negative distances).
  pub fn ratio(&self) -> Result<f64, TakeoffError> {
    self.validate()?;
    Ok(self.pixel_distance / self.real_distance)
  }
}

#[napi(discriminant = "type")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Scale {
  Area {
    id: String,
    page_id: String,
    scale: ScaleDefinition,
    bounding_box: (Point, Point),
  },
  Default {
    id: String,
    page_id: String,
    scale: ScaleDefinition,
  },
}

impl Scale {
  pub fn id(&self) -> String {
    match self {
      Scale::Area { id, .. } => id.clone(),
      Scale::Default { id, .. } => id.clone(),
    }
  }

  pub fn page_id(&self) -> String {
    match self {
      Scale::Area { page_id, .. } => page_id.clone(),
      Scale::Default { page_id, .. } => page_id.clone(),
    }
  }

  pub fn bounding_box_to_polygon(&self) -> Option<GeoPolygon<f64>> {
    match self {
      Scale::Area { bounding_box, .. } => {
        let start: Coord<f64> = bounding_box.0.into();
        let end: Coord<f64> = bounding_box.1.into();
        let rect = Rect::new(start, end);
        Some(rect.to_polygon())
      }
      _ => None,
    }
  }

  pub fn get_unit(&self) -> Unit {
    match self {
      Scale::Area { scale, .. } => scale.unit,
      Scale::Default { scale, .. } => scale.unit,
    }
  }

  pub fn is_in_bounding_box(&self, geometry: &Geometry<f64>) -> bool {
    match self {
      Scale::Area { .. } => {
        if let Some(polygon) = self.bounding_box_to_polygon() {
          polygon.contains(geometry)
        } else {
          false
        }
      }
      _ => false,
    }
  }

  /// Calculate the scale ratio (pixel_distance / real_distance).
  ///
  /// Returns an error if the scale is invalid (zero or negative distances).
  pub fn ratio(&self) -> Result<f64, TakeoffError> {
    match self {
      Scale::Area { scale, .. } => scale.ratio(),
      Scale::Default { scale, .. } => scale.ratio(),
    }
  }

  /// Validate that the scale has valid values.
  ///
  /// Returns an error if the scale definition is invalid.
  pub fn validate(&self) -> Result<(), TakeoffError> {
    match self {
      Scale::Area { scale, .. } => scale.validate(),
      Scale::Default { scale, .. } => scale.validate(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::error::TakeoffError;

  #[test]
  fn test_valid_scale_definition() {
    let scale_def = ScaleDefinition {
      pixel_distance: 100.0,
      real_distance: 10.0,
      unit: Unit::Feet,
    };
    assert!(scale_def.validate().is_ok());
    assert_eq!(scale_def.ratio().unwrap(), 10.0);
  }

  #[test]
  fn test_zero_pixel_distance_error() {
    let scale_def = ScaleDefinition {
      pixel_distance: 0.0,
      real_distance: 10.0,
      unit: Unit::Feet,
    };
    assert!(matches!(
      scale_def.validate(),
      Err(TakeoffError::InvalidScale { .. })
    ));
    assert!(matches!(
      scale_def.ratio(),
      Err(TakeoffError::InvalidScale { .. })
    ));
  }

  #[test]
  fn test_negative_pixel_distance_error() {
    let scale_def = ScaleDefinition {
      pixel_distance: -10.0,
      real_distance: 10.0,
      unit: Unit::Feet,
    };
    assert!(matches!(
      scale_def.validate(),
      Err(TakeoffError::InvalidScale { .. })
    ));
  }

  #[test]
  fn test_zero_real_distance_error() {
    let scale_def = ScaleDefinition {
      pixel_distance: 100.0,
      real_distance: 0.0,
      unit: Unit::Feet,
    };
    assert!(matches!(
      scale_def.validate(),
      Err(TakeoffError::InvalidScale { .. })
    ));
    assert!(matches!(
      scale_def.ratio(),
      Err(TakeoffError::InvalidScale { .. })
    ));
  }

  #[test]
  fn test_negative_real_distance_error() {
    let scale_def = ScaleDefinition {
      pixel_distance: 100.0,
      real_distance: -10.0,
      unit: Unit::Feet,
    };
    assert!(matches!(
      scale_def.validate(),
      Err(TakeoffError::InvalidScale { .. })
    ));
  }

  #[test]
  fn test_scale_ratio() {
    let scale = Scale::Default {
      id: "1".to_string(),
      page_id: "1".to_string(),
      scale: ScaleDefinition {
        pixel_distance: 120.0,
        real_distance: 1.0,
        unit: Unit::Feet,
      },
    };
    assert_eq!(scale.ratio().unwrap(), 120.0);
  }

  #[test]
  fn test_scale_validation() {
    let valid_scale = Scale::Default {
      id: "1".to_string(),
      page_id: "1".to_string(),
      scale: ScaleDefinition {
        pixel_distance: 100.0,
        real_distance: 10.0,
        unit: Unit::Feet,
      },
    };
    assert!(valid_scale.validate().is_ok());

    let invalid_scale = Scale::Default {
      id: "1".to_string(),
      page_id: "1".to_string(),
      scale: ScaleDefinition {
        pixel_distance: 0.0,
        real_distance: 10.0,
        unit: Unit::Feet,
      },
    };
    assert!(matches!(
      invalid_scale.validate(),
      Err(TakeoffError::InvalidScale { .. })
    ));
  }
}
