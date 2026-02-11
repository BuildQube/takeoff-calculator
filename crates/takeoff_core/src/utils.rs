use crate::coords::Point;
use crate::measurement::Measurement;
use geo::LineString;
use geo::Simplify;
use napi::bindgen_prelude::Result;
use napi_derive::napi;

/// Simplify a polyline using the Ramer-Douglas-Peucker algorithm
#[napi]
pub fn simplify_polyline(points: Vec<Point>, tolerance: f64) -> Vec<Point> {
  let line_string = LineString::new(points.iter().map(|p| (*p).into()).collect());

  let simplified = line_string.simplify(tolerance);
  simplified.into_iter().map(Point::from).collect()
}

/// Get the centroid of a measurement
///
/// Returns `None` if the measurement has invalid geometry.
/// For more detailed error information, use `measurement.get_centroid()` directly.
#[napi]
pub fn get_centroid(measurement: Measurement) -> Option<Point> {
  measurement.get_centroid().ok()
}

/// Reposition a measurement so its centroid is at the given point.
/// Returns a new measurement (same kind and metadata); area, length, and count are unchanged.
///
/// # Errors
///
/// Returns an error if the measurement has invalid or empty geometry (e.g. `EmptyGeometry`).
#[napi]
pub fn reposition_measurement_to_centroid(
  measurement: Measurement,
  new_centroid: Point,
) -> Result<Measurement> {
  measurement
    .with_centroid_at(new_centroid)
    .map_err(Into::into)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_simplify_polyline() {
    let points = vec![
      Point::new(0.0, 0.0),
      Point::new(1.0, 0.0),
      Point::new(1.0, 1.0),
      Point::new(2.0, 2.0),
    ];
    let simplified = simplify_polyline(points, 0.5);
    assert_eq!(
      simplified,
      vec![
        Point::new(0.0, 0.0),
        Point { x: 1.0, y: 0.0 },
        Point::new(2.0, 2.0)
      ]
    );
  }

  #[test]
  fn test_get_centroid() {
    let measurement = Measurement::Rectangle {
      id: "1".to_string(),
      page_id: "1".to_string(),
      group_id: "1".to_string(),
      points: (Point::new(0.0, 0.0), Point::new(1.0, 1.0)),
    };

    let centroid = get_centroid(measurement);
    assert_eq!(centroid, Some(Point::new(0.5, 0.5)));
  }

  #[test]
  fn test_get_centroid_invalid() {
    let measurement = Measurement::Polygon {
      id: "1".to_string(),
      page_id: "1".to_string(),
      group_id: "1".to_string(),
      points: vec![Point::new(0.0, 0.0), Point::new(1.0, 0.0)], // Only 2 points
    };

    let centroid = get_centroid(measurement);
    assert_eq!(centroid, None);
  }

  #[test]
  fn test_reposition_measurement_to_centroid() {
    let measurement = Measurement::Rectangle {
      id: "1".to_string(),
      page_id: "1".to_string(),
      group_id: "1".to_string(),
      points: (Point::new(0.0, 0.0), Point::new(2.0, 2.0)),
    };
    let new_centroid = Point::new(10.0, 20.0);
    let result = reposition_measurement_to_centroid(measurement, new_centroid).unwrap();
    let got = result.get_centroid().unwrap();
    assert!((got.x - new_centroid.x).abs() < 1e-10 && (got.y - new_centroid.y).abs() < 1e-10);
  }

  #[test]
  fn test_reposition_measurement_to_centroid_invalid() {
    let measurement = Measurement::Polygon {
      id: "1".to_string(),
      page_id: "1".to_string(),
      group_id: "1".to_string(),
      points: vec![Point::new(0.0, 0.0), Point::new(1.0, 0.0)],
    };
    let result = reposition_measurement_to_centroid(measurement, Point::new(0.0, 0.0));
    assert!(result.is_err(), "empty geometry should yield error");
  }
}
