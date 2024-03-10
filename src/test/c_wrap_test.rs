#[cfg(test)]
mod tests {
    use crate::sdf::{c_wrap::*, sdf_type::{Marchable, SDFCapsule}};
    use glm::DVec2;

  #[test]
  fn init_free_wrap() {
    unsafe {
      let b = bundle_get(0.0);

      bundle_add_circle(b, 0.0, 0.0, 1.0);
      let mut test_res = bundle_dist(b, 1.0, 0.0);

      assert_eq!(test_res, 0.0, "dist not expected!");
      test_res = bundle_dist(b, 0.0, 1.0);
      assert_eq!(test_res, 0.0, "dist not expected!");

      test_res = bundle_dist(b, 0.0, 0.0);
      assert_eq!(test_res, -1.0, "dist not expected!");
      bundle_free(b);
    }
  }

  #[test]
  fn capsule_test() {
    let c = SDFCapsule::new_variable(
      vec![DVec2::new(0.0, 0.0), DVec2::new(5.0, 0.0)],
      vec![2.0, 6.0]
    );

    let mut v = DVec2::new(1.5, 0.1);
    // frustum shape
    assert!(c.dist(&v) < 0.0);
    
    v.y = 4.5;
    assert!(c.dist(&v) > 0.0);

    v.x = 4.9;
    assert!(c.dist(&v) < 0.0);

    v.y = -7.0;
    assert!(c.dist(&v) > 0.0);
  }
}
