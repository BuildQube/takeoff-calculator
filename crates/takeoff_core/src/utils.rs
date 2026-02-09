use crate::coords::Point;
use geo::LineString;
use geo::Simplify;

use napi_derive::napi;

#[napi]
/// Simplify a polyline using the Ramer-Douglas-Peucker algorithm
pub fn simplify_polyline(points: Vec<Point>, tolerance: f64) -> Vec<Point> {
  let line_string = LineString::new(points.iter().map(|p| (*p).into()).collect());

  let simplified = line_string.simplify(tolerance);
  simplified.into_iter().map(Point::from).collect()
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
}
