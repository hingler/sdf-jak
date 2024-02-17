use crate::sdf::sdf_type::*;
use std::vec::Vec;
use glm::DVec2;

pub struct SDFBundle {
  circles: Vec<SDFCircle>,
  capsules: Vec<SDFCapsule>
}

// how to impl btwn c : rust?
// - export ptr to bundle
// - call c-like (ie pass ptr as first argument)

impl SDFBundle {
  pub fn new() -> Self {
    return SDFBundle {
      circles: Vec::new(),
      capsules: Vec::new()
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

impl Marchable for SDFBundle {
  fn dist(&self, point: &DVec2) -> f64 {
    let mut min_dist = f64::MAX;
    for c in &self.circles {
      min_dist = f64::min(min_dist, c.dist(point));
    }

    for c in &self.capsules {
      min_dist = f64::min(min_dist, c.dist(point));
    }

    return min_dist;
  }
}