use crate::sdf::bundle::SDFBundle;
use crate::sdf::sdf_type::Marchable;

use std::mem;
use libc::malloc;

use libc::{c_uint, c_double};

#[no_mangle]
pub unsafe extern "C" fn bundle_get() -> *mut SDFBundle {
  return malloc(mem::size_of::<SDFBundle>()) as *mut SDFBundle;
}

#[no_mangle]
pub unsafe extern "C" fn bundle_add_circle(bundle: *mut SDFBundle, x: c_double, y: c_double, radius: c_double) {
  (*bundle).add_circle(&x, &y, &radius);
}

#[no_mangle]
pub unsafe extern "C" fn bundle_add_capsule(bundle: *mut SDFBundle, points: *mut c_double, point_count: c_uint, radius: c_double) {
  let points_dvec = points as *mut glm::DVec2;
  let mut point_vec = Vec::new();
  let mut point_cur: *mut glm::DVec2 = points_dvec;
  for _ in 0..point_count {
    // write manually lole
    point_vec.push(*points_dvec);
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