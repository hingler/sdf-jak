#[cfg(test)]
mod tests {
    use crate::sdf::c_wrap::*;

  #[test]
  fn init_free_wrap() {
    unsafe {
      let b = bundle_get();

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
}