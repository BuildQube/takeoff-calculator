use crate::coords::{Point, Point3D};
use napi_derive::napi;
use serde::{Deserialize, Serialize};

#[napi(object)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContourLineInput {
  pub elevation: f64,
  pub points: Vec<Point>,
}

impl From<ContourLineInput> for Vec<Point3D> {
  fn from(input: ContourLineInput) -> Self {
    input
      .points
      .into_iter()
      .map(|p| Point3D {
        x: p.x,
        y: p.y,
        z: input.elevation,
      })
      .collect()
  }
}

#[napi(object)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContourPointOfInterestInput {
  pub elevation: f64,
  pub point: Point,
}

impl From<ContourPointOfInterestInput> for Point3D {
  fn from(input: ContourPointOfInterestInput) -> Self {
    Point3D {
      x: input.point.x,
      y: input.point.y,
      z: input.elevation,
    }
  }
}

#[napi(object)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContourInput {
  pub id: String,
  pub name: Option<String>,

  /// The lines that make up the contour map
  pub lines: Vec<ContourLineInput>,
  /// The points of interest that are used to create the contour map
  pub points_of_interest: Vec<ContourPointOfInterestInput>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContourMesh {
  pub id: String,
  pub name: Option<String>,
  pub points: Vec<Point3D>,
}

impl From<ContourInput> for ContourMesh {
  fn from(input: ContourInput) -> Self {
    let mut points: Vec<Point3D> = Vec::new();

    for line in input.lines {
      let line_points: Vec<Point3D> = line.into();
      points.extend(line_points);
    }
    for point_of_interest in input.points_of_interest {
      let point_of_interest_point: Point3D = point_of_interest.into();
      points.push(point_of_interest_point);
    }

    Self {
      id: input.id,
      name: input.name,
      points,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_contour_mesh() {
    let input = ContourInput {
      id: "1".to_string(),
      name: Some("test".to_string()),
      lines: vec![ContourLineInput {
        elevation: 10.0,
        points: vec![Point::new(0.0, 0.0), Point::new(10.0, 0.0)],
      }],
      points_of_interest: vec![ContourPointOfInterestInput {
        elevation: 5.0,
        point: Point::new(5.0, 5.0),
      }],
    };
    let contour_mesh: ContourMesh = input.into();
    assert_eq!(contour_mesh.id, "1");
    assert_eq!(contour_mesh.name, Some("test".to_string()));
    assert_eq!(contour_mesh.points.len(), 3);
    assert_eq!(
      contour_mesh.points,
      vec![
        Point3D {
          x: 0.0,
          y: 0.0,
          z: 10.0
        },
        Point3D {
          x: 10.0,
          y: 0.0,
          z: 10.0
        },
        Point3D {
          x: 5.0,
          y: 5.0,
          z: 5.0
        },
      ]
    );
  }
}
