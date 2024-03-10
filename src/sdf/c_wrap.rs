use crate::sdf::bundle::SDFBundle;
use crate::sdf::sdf_type::Marchable;

use libc::{c_uint, c_double};

use super::smooth::SDFSmooth;

type SDFSmoother = SDFSmooth<SDFBundle, SDFBundle>;

/// SMOOTHER FUNCS
#[no_mangle]
pub unsafe extern "C" fn smoother_get(a: *mut SDFBundle, b: *mut SDFBundle, k: c_double) -> *mut SDFSmoother {
  // how to do???
  let smoother = Box::new(SDFSmoother::new((*a).clone(), (*b).clone(), k));
  return Box::into_raw(smoother);
}

#[no_mangle]
pub unsafe extern "C" fn smoother_dist(s: *mut SDFSmoother, x: c_double, y: c_double) -> f64 {
 let pt = glm::dvec2(x, y);
 return (*s).dist(&pt);
}

#[no_mangle]
pub unsafe extern "C" fn smoother_free(s: *mut SDFSmoother) {
  drop(Box::from_raw(s));
}

/// BUNDLE FUNCS
#[no_mangle]
pub unsafe extern "C" fn bundle_get(k: c_double) -> *mut SDFBundle {
  return Box::into_raw(Box::new(SDFBundle::new_s(k)));
}

#[no_mangle]
pub unsafe extern "C" fn bundle_copy(bundle: *mut SDFBundle) -> *mut SDFBundle {
  return Box::into_raw(Box::new(SDFBundle::copy((*bundle).clone())));
}

#[no_mangle]
pub unsafe extern "C" fn bundle_add_circle(bundle: *mut SDFBundle, x: c_double, y: c_double, radius: c_double) {
  (*bundle).add_circle(&x, &y, &radius);
}

#[no_mangle]
pub unsafe extern "C" fn bundle_add_capsule(bundle: *mut SDFBundle, points: *mut c_double, point_count: c_uint, radius: c_double) {
  let points_dvec = points as *mut glm::DVec2;

  let mut point_vec = Vec::new();
  // works - repr c on dvec2
  let mut point_cur: *mut glm::DVec2 = points_dvec;
  for _ in 0..point_count {
    // write manually lole
    point_vec.push(*point_cur);
    point_cur = point_cur.add(1);
  }

  (*bundle).add_capsule_move(point_vec, radius);
}

#[no_mangle]
pub unsafe extern "C" fn bundle_dist(bundle: *mut SDFBundle, x: c_double, y: c_double) -> c_double {
  let pt = glm::dvec2(x, y);
  return (*bundle).dist(&pt);
}

#[no_mangle]
pub unsafe extern "C" fn bundle_free(bundle: *mut SDFBundle) {
  drop(Box::from_raw(bundle));
}
