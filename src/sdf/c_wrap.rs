use crate::sdf::bundle::SDFBundle;

pub unsafe extern "C" fn get_bundle() -> *mut SDFBundle {
  let bundle_ptr = Box::into_raw(Box::new(SDFBundle::new()));
  return bundle_ptr;
}

pub unsafe extern "C" fn bundle_add_circle(bundle: *mut SDFBundle, x: f64, y: f64, radius: f64) {
  (*bundle).add_circle(&x, &y, &radius);
}

pub unsafe extern "C" fn bundle_add_capsule(bundle: *mut SDFBundle, points: *mut glm::DVec2, point_count: usize, radius: f64) {
  let mut point_vec = Vec::new();
  let mut point_cur: *mut glm::DVec2 = points;
  for _ in 0..point_count {
    // write manually lole
    point_vec.push(*points);
    point_cur = point_cur.add(1);
  }

  (*bundle).add_capsule_move(point_vec, radius);
}