use crate::sdf::sdf_type::*;
use std::{ops::Neg, vec::Vec};
use glm::DVec2;

#[derive(Clone)]
#[repr(C)]
pub struct SDFBundle {
  circles: Vec<SDFCircle>,
  capsules: Vec<SDFCapsule>,
  fac: f64
}

impl Neg for SDFBundle {
  type Output = SDFBundle;

  fn neg(mut self) -> Self::Output {
    self.fac *= -1.0f64;
    return self;
  }
}

// how to impl btwn c : rust?
// - export ptr to bundle
// - call c-like (ie pass ptr as first argument)

impl SDFBundle {
  pub fn new() -> Self {
    return SDFBundle {
      circles: Vec::new(),
      capsules: Vec::new(),
      fac: 1.0
    };
  }

  pub fn add_circle(&mut self, x: &f64, y: &f64, radius: &f64) {
    self.circles.push(SDFCircle::new(x, y, radius));
  }

  pub fn add_capsule(&mut self, points: &Vec<DVec2>, radius: f64) {
    self.capsules.push(
      SDFCapsule::new(points, &radius)
    );
  }

  pub fn add_capsule_move(&mut self, points: Vec<DVec2>, radius: f64) {
    self.capsules.push(
      SDFCapsule::new_move(points, &radius)
    );
  }
}

// how does this work when we introduce smoothing?
// - i think we would smooth bundle-pairs on the c side

impl Marchable for SDFBundle {
  fn dist(&self, point: &DVec2) -> f64 {
    let mut min_dist = f64::MAX;
    for c in &self.circles {
      min_dist = f64::min(min_dist, c.dist(point));
    }

    for c in &self.capsules {
      min_dist = f64::min(min_dist, c.dist(point));
    }

    // hehe smile
    return min_dist * self.fac;
  }
}