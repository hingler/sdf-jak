use crate::sdf::sdf_type::*;
use std::{ops::Neg, vec::Vec};
use glm::DVec2;

use super::smooth::smin_f;

#[derive(Clone)]
#[repr(C)]
pub struct SDFBundle {
  circles: Vec<SDFCircle>,
  capsules: Vec<SDFCapsule>,
  k: f64,
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

const EPSILON_K: f64 = 0.00001;

impl SDFBundle {
  pub fn new() -> Self {
    return SDFBundle::new_s(0.0f64);
  }

  pub fn copy(bundle: SDFBundle) -> Self {
    return SDFBundle {
      circles: bundle.circles.clone(),
      capsules: bundle.capsules.clone(),
      k: bundle.k,
      fac: bundle.fac
    }
  }

  pub fn new_s(k_v: f64) -> Self {

    return SDFBundle {
      circles: Vec::new(),
      capsules: Vec::new(),
      k: f64::max(k_v, EPSILON_K * 0.5),   // eff: 0
      fac: 1.0
    };
  }

  pub fn add_circle(&mut self, x: &f64, y: &f64, radius: &f64) {
    self.circles.push(SDFCircle::new(x, y, radius));
  }

  pub fn add_capsule(&mut self, points: &Vec<DVec2>, radius: f64) {
    println!("point count: {}", points.len());
    self.capsules.push(
      SDFCapsule::new(points, &radius)
    );
  }

  pub fn add_capsule_var(&mut self, points: Vec<DVec2>, rads: Vec<f64>) {
    self.capsules.push(
      SDFCapsule::new_variable(points, rads)
    );
  }

  pub fn add_capsule_move(&mut self, points: Vec<DVec2>, radius: f64) {
    self.capsules.push(
      SDFCapsule::new_move(points, &radius)
    );
  }

  pub fn dist_s(&self, point: &glm::DVec2) -> f64 {
    let mut dist = f64::MAX;
    for circle in &self.circles {
      dist = smin_f(dist, circle.dist(point), self.k);
    }

    for capsule in &self.capsules {
      dist = smin_f(dist, capsule.dist(point), self.k);
    }

    return dist * self.fac;

  }

  fn dist_n(&self, point: &glm::DVec2) -> f64 {
    let mut dist = f64::MAX;
    for circle in &self.circles {
      dist = f64::min(dist, circle.dist(point));
    }

    for capsule in &self.capsules {
      dist = f64::min(dist, capsule.dist(point));
    }

    return dist;
  }
}

// how does this work when we introduce smoothing?
// - i think we would smooth bundle-pairs on the c side

impl Marchable for SDFBundle {
  fn dist(&self, point: &glm::DVec2) -> f64 {
    // slow if check?????
    if self.k < EPSILON_K {
      return self.dist_n(point);
    } else {
      return self.dist_s(point);
    }
  }
}
