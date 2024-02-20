#include <gtest/gtest.h>

#include "sdf_jak/sdf_bundle.h"

TEST(BundleTest, CreateBundle) {
  SDFBundle* bundle = bundle_get();
  bundle_add_circle(bundle, 0.0, 0.0, 4.0);
  ASSERT_NEAR(bundle_dist(bundle, 0.0, 4.0), 0.0, 0.0001);

  ASSERT_NEAR(bundle_dist(bundle, 2.0, 0.0), -2.0, 0.0001);

  bundle_free(bundle);
}