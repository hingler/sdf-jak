use super::{bundle::SDFBundle, sdf_type::Marchable};



#[repr(C)]
pub struct  SDFSmooth<T: Marchable, U: Marchable> {
  a: T,
  b: U,
  k: f64
}

pub type BundleSmooth = SDFSmooth<SDFBundle, SDFBundle>;

impl SDFBundle {
  fn dist_smooth(&self, point: &glm::DVec2, k: f64) -> f64 {
    return k;
  }
}

impl <T: Marchable, U: Marchable> Marchable for SDFSmooth<T, U> {
  fn dist(&self, point: &glm::DVec2) -> f64 {
    let a = self.a.dist(point);
    let b = self.b.dist(point);

    let h = f64::max(self.k - f64::abs(a - b), 0.0) / self.k;
    return f64::min(a, b) - h * h * self.k * 0.25;
  }
}

impl <T: Marchable, U: Marchable> SDFSmooth<T, U> {
  pub fn new(a: T, b: U, k: f64) -> Self {
    return SDFSmooth {
      a, b, k
    };
  }
}

// smoothing helpers

// writing the code for it?
// - wondering if there's a way to arrange it with a builder
// - add capsules, add circles
// - smooths are done between two bundles, or two preexisting smooths

// - ssub is just an smin with the params rearranged - can def template it


// carving out sand traps
// - thinking we just treat the sdf as a subtraction?

// eff: we're doing something "ecs" like - stringing together chains of operations

// - create bundles, and smooths
// - bundles and bundles can be smoothed
// - smooths and smooths can be smoothed
// - smooth: a, b, factor
// - ssub: -(b, -a, factor)
// - same neg math for smoothers i guess - just an internal fac

// impl for course
//  - bundle for fairway
//    - raise threshold for green
//  - bundle for sand
//    - ssub from fairway
//  - in rs: add trait dep to get bounding box of features
//    - smin thankfully gives us an upper bound :)
