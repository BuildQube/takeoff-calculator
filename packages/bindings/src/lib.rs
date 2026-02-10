#![deny(clippy::all)]

pub mod contour;
pub mod group;
pub mod measurement;
pub mod state;
use napi_derive::napi;

#[napi]
pub fn plus_100(input: u32) -> u32 {
  input + 100
}

#[napi]
pub fn plus_200(input: u32) -> u32 {
  input + 200
}
