#ifndef SDF_BUNDLE_H_
#define SDF_BUNDLE_H_

// test wrapper for sdf bundle :3
// (tba: raii cpp wrap :o)

#ifdef __cplusplus
extern "C" {
#endif

struct SDFBundle;
struct SDFSmoother;

// theres basically no point in this except to have fun :)
// integrate in cpp and test

SDFSmoother* smoother_get(SDFBundle* a, SDFBundle* b, double k);

double smoother_dist(SDFSmoother* s, double x, double y);

void smoother_free(SDFSmoother* smoother);

SDFBundle* bundle_get(double k);

SDFBundle* bundle_copy(SDFBundle* bundle);

// swag!
// we're just having fun :3

// creates a new circle
void bundle_add_circle(SDFBundle* bundle, double x, double y, double radius);

// creates a new capsule
void bundle_add_capsule(SDFBundle* bundle, const double* points, unsigned int point_count, double radius);

// creates a new capsule w variable width
void bundle_add_capsule_var(SDFBundle* bundle, const double* points, const double* rads, unsigned int point_count);

double bundle_dist(SDFBundle* bundle, double x, double y);

void bundle_free(SDFBundle* bundle);

#ifdef __cplusplus
}
#endif

#endif // SDF_BUNDLE_H_
