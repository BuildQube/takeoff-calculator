use crate::error::TakeoffError;
use napi_derive::napi;
use serde::{Deserialize, Serialize};
use uom::fmt::DisplayStyle::Abbreviation;
use uom::si::area::{square_centimeter, square_foot, square_inch, square_meter, square_yard};
use uom::si::f32::{Area, Length, Volume};
use uom::si::length::{centimeter, foot, inch, meter, yard};
use uom::si::volume::{cubic_centimeter, cubic_foot, cubic_inch, cubic_meter, cubic_yard};
/// Measurement units supported by the system
#[napi(string_enum)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Unit {
  /// Imperial units
  Yards,
  Feet,
  Inches,
  /// Metric units
  Meters,
  Centimeters,
}

impl Unit {
  pub fn convert_length_to_unit(&self, length: Length) -> f32 {
    match self {
      Unit::Yards => length.get::<yard>(),
      Unit::Feet => length.get::<foot>(),
      Unit::Inches => length.get::<inch>(),
      Unit::Meters => length.get::<meter>(),
      Unit::Centimeters => length.get::<centimeter>(),
    }
  }
  pub fn get_unit(&self, value: f32) -> Length {
    match self {
      Unit::Yards => Length::new::<yard>(value),
      Unit::Feet => Length::new::<foot>(value),
      Unit::Inches => Length::new::<inch>(value),
      Unit::Meters => Length::new::<meter>(value),
      Unit::Centimeters => Length::new::<centimeter>(value),
    }
  }

  pub fn convert_area_to_unit(&self, area: Area) -> f32 {
    match self {
      Unit::Yards => area.get::<square_yard>(),
      Unit::Feet => area.get::<square_foot>(),
      Unit::Inches => area.get::<square_inch>(),
      Unit::Meters => area.get::<square_meter>(),
      Unit::Centimeters => area.get::<square_centimeter>(),
    }
  }

  pub fn get_area_unit(&self, value: f32) -> Area {
    match self {
      Unit::Yards => Area::new::<square_yard>(value),
      Unit::Feet => Area::new::<square_foot>(value),
      Unit::Inches => Area::new::<square_inch>(value),
      Unit::Meters => Area::new::<square_meter>(value),
      Unit::Centimeters => Area::new::<square_centimeter>(value),
    }
  }

  pub fn convert_volume_to_unit(&self, volume: Volume) -> f32 {
    match self {
      Unit::Yards => volume.get::<cubic_yard>(),
      Unit::Feet => volume.get::<cubic_foot>(),
      Unit::Inches => volume.get::<cubic_inch>(),
      Unit::Meters => volume.get::<cubic_meter>(),
      Unit::Centimeters => volume.get::<cubic_centimeter>(),
    }
  }

  pub fn get_volume_unit(&self, value: f32) -> Volume {
    match self {
      Unit::Yards => Volume::new::<cubic_yard>(value),
      Unit::Feet => Volume::new::<cubic_foot>(value),
      Unit::Inches => Volume::new::<cubic_inch>(value),
      Unit::Meters => Volume::new::<cubic_meter>(value),
      Unit::Centimeters => Volume::new::<cubic_centimeter>(value),
    }
  }

  /// Convert a value from one unit to another
  pub fn convert(&self, value: f32, to: &Unit) -> f32 {
    let from = self.get_unit(value);

    match to {
      Unit::Yards => from.get::<yard>(),
      Unit::Feet => from.get::<foot>(),
      Unit::Inches => from.get::<inch>(),
      Unit::Meters => from.get::<meter>(),
      Unit::Centimeters => from.get::<centimeter>(),
    }
  }

  pub fn convert_area(&self, value: f32, to: &Unit) -> f32 {
    let from = self.get_area_unit(value);

    match to {
      Unit::Yards => from.get::<square_yard>(),
      Unit::Feet => from.get::<square_foot>(),
      Unit::Inches => from.get::<square_inch>(),
      Unit::Meters => from.get::<square_meter>(),
      Unit::Centimeters => from.get::<square_centimeter>(),
    }
  }

  pub fn convert_volume(&self, value: f32, to: &Unit) -> f32 {
    let from = self.get_volume_unit(value);

    match to {
      Unit::Yards => from.get::<cubic_yard>(),
      Unit::Feet => from.get::<cubic_foot>(),
      Unit::Inches => from.get::<cubic_inch>(),
      Unit::Meters => from.get::<cubic_meter>(),
      Unit::Centimeters => from.get::<cubic_centimeter>(),
    }
  }

  /// Get the display string for this unit
  pub fn display(&self) -> &'static str {
    match self {
      Unit::Yards => "yd",
      Unit::Feet => "ft",
      Unit::Inches => "in",
      Unit::Meters => "m",
      Unit::Centimeters => "cm",
    }
  }

  pub fn unit_str(&self) -> &'static str {
    match self {
      Unit::Yards => "Yards",
      Unit::Feet => "Feet",
      Unit::Inches => "Inches",
      Unit::Meters => "Meters",
      Unit::Centimeters => "Centimeters",
    }
  }

  /// Parse a unit string into a Unit enum.
  ///
  /// Returns an error if the string is not recognized as a supported unit.
  /// Case-insensitive matching is performed.
  pub fn from_str(s: &str) -> Result<Unit, TakeoffError> {
    match s.to_lowercase().as_str() {
      "yards" | "yard" | "yd" => Ok(Unit::Yards),
      "feet" | "foot" | "ft" => Ok(Unit::Feet),
      "inches" | "inch" | "in" => Ok(Unit::Inches),
      "meters" | "meter" | "m" => Ok(Unit::Meters),
      "centimeters" | "centimeter" | "cm" => Ok(Unit::Centimeters),
      _ => Err(TakeoffError::unknown_unit(s.to_string())),
    }
  }
}

/// Unit conversion utilities
pub struct UnitUtils;

impl UnitUtils {
  /// Convert a value from one unit to another
  pub fn convert(value: f32, from: Unit, to: Unit) -> f32 {
    from.convert(value, &to)
  }
  pub fn convert_area(value: f32, from: Unit, to: Unit) -> f32 {
    from.convert_area(value, &to)
  }

  pub fn convert_volume(value: f32, from: Unit, to: Unit) -> f32 {
    from.convert_volume(value, &to)
  }

  /// Get all available units
  pub fn all_units() -> Vec<Unit> {
    vec![
      Unit::Yards,
      Unit::Feet,
      Unit::Inches,
      Unit::Meters,
      Unit::Centimeters,
    ]
  }

  /// Get imperial units
  pub fn imperial_units() -> Vec<Unit> {
    vec![Unit::Yards, Unit::Feet, Unit::Inches]
  }

  /// Get metric units
  pub fn metric_units() -> Vec<Unit> {
    vec![Unit::Meters, Unit::Centimeters]
  }
}

pub enum UnitFormatter {
  Length { unit: Unit, value: f32 },
  Area { unit: Unit, value: f32 },
  Volume { unit: Unit, value: f32 },
}

impl UnitFormatter {
  pub fn format(&self) -> String {
    match self {
      UnitFormatter::Area {
        unit: Unit::Yards,
        value,
      } => Unit::Yards
        .get_area_unit(*value)
        .into_format_args(square_yard, Abbreviation)
        .to_string(),
      UnitFormatter::Area {
        unit: Unit::Feet,
        value,
      } => Unit::Feet
        .get_area_unit(*value)
        .into_format_args(square_foot, Abbreviation)
        .to_string(),
      UnitFormatter::Area {
        unit: Unit::Inches,
        value,
      } => Unit::Inches
        .get_area_unit(*value)
        .into_format_args(square_inch, Abbreviation)
        .to_string(),
      UnitFormatter::Area {
        unit: Unit::Meters,
        value,
      } => Unit::Meters
        .get_area_unit(*value)
        .into_format_args(square_meter, Abbreviation)
        .to_string(),
      UnitFormatter::Area {
        unit: Unit::Centimeters,
        value,
      } => Unit::Centimeters
        .get_area_unit(*value)
        .into_format_args(square_centimeter, Abbreviation)
        .to_string(),
      UnitFormatter::Length {
        unit: Unit::Yards,
        value,
      } => Unit::Yards
        .get_unit(*value)
        .into_format_args(yard, Abbreviation)
        .to_string(),
      UnitFormatter::Length {
        unit: Unit::Feet,
        value,
      } => Unit::Feet
        .get_unit(*value)
        .into_format_args(foot, Abbreviation)
        .to_string(),
      UnitFormatter::Length {
        unit: Unit::Inches,
        value,
      } => Unit::Inches
        .get_unit(*value)
        .into_format_args(inch, Abbreviation)
        .to_string(),
      UnitFormatter::Length {
        unit: Unit::Meters,
        value,
      } => Unit::Meters
        .get_unit(*value)
        .into_format_args(meter, Abbreviation)
        .to_string(),
      UnitFormatter::Length {
        unit: Unit::Centimeters,
        value,
      } => Unit::Centimeters
        .get_unit(*value)
        .into_format_args(centimeter, Abbreviation)
        .to_string(),
      UnitFormatter::Volume {
        unit: Unit::Yards,
        value,
      } => Unit::Yards
        .get_volume_unit(*value)
        .into_format_args(cubic_yard, Abbreviation)
        .to_string(),
      UnitFormatter::Volume {
        unit: Unit::Feet,
        value,
      } => Unit::Feet
        .get_volume_unit(*value)
        .into_format_args(cubic_foot, Abbreviation)
        .to_string(),
      UnitFormatter::Volume {
        unit: Unit::Inches,
        value,
      } => Unit::Inches
        .get_volume_unit(*value)
        .into_format_args(cubic_inch, Abbreviation)
        .to_string(),
      UnitFormatter::Volume {
        unit: Unit::Meters,
        value,
      } => Unit::Meters
        .get_volume_unit(*value)
        .into_format_args(cubic_meter, Abbreviation)
        .to_string(),
      UnitFormatter::Volume {
        unit: Unit::Centimeters,
        value,
      } => Unit::Centimeters
        .get_volume_unit(*value)
        .into_format_args(cubic_centimeter, Abbreviation)
        .to_string(),
    }
  }
}

// #[napi(discriminant = "type")]
// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub enum UnitValue {
//   Area { value: Area },
//   Length { value: Length },
// }

// #[napi(discriminant = "type")]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum UnitValueItem {
  Area { value: Area },
  Length { value: Length },
  Volume { value: Volume },
}

#[napi(string_enum)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum UnitValueItemType {
  Area,
  Length,
  Volume,
}

#[napi]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitValue {
  value: UnitValueItem,
}

#[napi]
impl UnitValue {
  #[napi(constructor)]
  pub fn new(value: f64, unit: Unit, magnitude: UnitValueItemType) -> Self {
    match magnitude {
      UnitValueItemType::Area => Self {
        value: UnitValueItem::Area {
          value: unit.get_area_unit(value as f32),
        },
      },
      UnitValueItemType::Length => Self {
        value: UnitValueItem::Length {
          value: unit.get_unit(value as f32),
        },
      },
      UnitValueItemType::Volume => Self {
        value: UnitValueItem::Volume {
          value: unit.get_volume_unit(value as f32),
        },
      },
    }
  }

  pub fn from_area(value: Area) -> Self {
    Self {
      value: UnitValueItem::Area { value },
    }
  }
  pub fn from_length(value: Length) -> Self {
    Self {
      value: UnitValueItem::Length { value },
    }
  }
  pub fn from_volume(value: Volume) -> Self {
    Self {
      value: UnitValueItem::Volume { value },
    }
  }

  #[napi]
  pub fn display(&self, unit: Unit) -> String {
    match self.value {
      UnitValueItem::Area { value } => UnitFormatter::Area {
        unit,
        value: unit.convert_area_to_unit(value),
      }
      .format(),
      UnitValueItem::Length { value } => UnitFormatter::Length {
        unit,
        value: unit.convert_length_to_unit(value),
      }
      .format(),
      UnitValueItem::Volume { value } => UnitFormatter::Volume {
        unit,
        value: unit.convert_volume_to_unit(value),
      }
      .format(),
    }
  }

  #[napi]
  pub fn get_converted_value(&self, to: Unit) -> f64 {
    match self.value {
      UnitValueItem::Area { value } => to.convert_area_to_unit(value) as f64,
      UnitValueItem::Length { value } => to.convert_length_to_unit(value) as f64,
      UnitValueItem::Volume { value } => to.convert_volume_to_unit(value) as f64,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_format() {
    let formatter = UnitFormatter::Length {
      unit: Unit::Meters,
      value: 1.0,
    };
    assert_eq!(formatter.format(), "1 m");
    let formatter = UnitFormatter::Area {
      unit: Unit::Meters,
      value: 1.0,
    };
    assert_eq!(formatter.format(), "1 m²");
  }

  #[test]
  fn test_convert() {
    let result = Unit::Yards.convert(1.0, &Unit::Feet);
    println!("result: {}", result);
    assert_eq!(result, 3.0);
  }

  #[test]
  fn test_convert_area() {
    let result = UnitUtils::convert_area(1.0, Unit::Meters, Unit::Feet);
    assert_eq!(result, 10.76391);
  }

  #[test]
  fn test_new_unit_value() {
    let unit_value = UnitValue::new(1.0, Unit::Meters, UnitValueItemType::Length);

    assert_eq!(unit_value.get_converted_value(Unit::Meters), 1.0);
    assert_eq!(unit_value.display(Unit::Meters), "1 m");
    let unit_value = UnitValue::new(1.0, Unit::Meters, UnitValueItemType::Area);

    assert_eq!(unit_value.display(Unit::Meters), "1 m²");
    assert_eq!(unit_value.get_converted_value(Unit::Meters), 1.0);

    let unit_value = UnitValue::new(1.0, Unit::Meters, UnitValueItemType::Volume);
    assert_eq!(unit_value.display(Unit::Meters), "1 m³");
    assert_eq!(unit_value.get_converted_value(Unit::Meters), 1.0);
  }

  #[test]
  fn test_unit_from_str() {
    assert_eq!(Unit::from_str("yards").unwrap(), Unit::Yards);
    assert_eq!(Unit::from_str("Yards").unwrap(), Unit::Yards);
    assert_eq!(Unit::from_str("YARDS").unwrap(), Unit::Yards);
    assert_eq!(Unit::from_str("yd").unwrap(), Unit::Yards);
    assert_eq!(Unit::from_str("feet").unwrap(), Unit::Feet);
    assert_eq!(Unit::from_str("ft").unwrap(), Unit::Feet);
    assert_eq!(Unit::from_str("inches").unwrap(), Unit::Inches);
    assert_eq!(Unit::from_str("in").unwrap(), Unit::Inches);
    assert_eq!(Unit::from_str("meters").unwrap(), Unit::Meters);
    assert_eq!(Unit::from_str("m").unwrap(), Unit::Meters);
    assert_eq!(Unit::from_str("centimeters").unwrap(), Unit::Centimeters);
    assert_eq!(Unit::from_str("cm").unwrap(), Unit::Centimeters);
  }

  #[test]
  fn test_unit_from_str_unknown() {
    assert!(matches!(
      Unit::from_str("kilometers"),
      Err(crate::error::TakeoffError::UnknownUnit { .. })
    ));
    assert!(matches!(
      Unit::from_str("miles"),
      Err(crate::error::TakeoffError::UnknownUnit { .. })
    ));
    assert!(matches!(
      Unit::from_str(""),
      Err(crate::error::TakeoffError::UnknownUnit { .. })
    ));
    assert!(matches!(
      Unit::from_str("invalid"),
      Err(crate::error::TakeoffError::UnknownUnit { .. })
    ));
  }
}
