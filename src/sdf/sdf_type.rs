use glm::{dvec2, sqrt, DVec2};

#[derive(Clone, Copy)]
pub struct SDFCircle {
  point: DVec2,
  radius: f64
}

fn closest_point(seg_a: &DVec2, seg_b: &DVec2, point: &DVec2) -> DVec2 {
  // b - a is line len
  // normalize
  let seg_rect = Rect::from(seg_a, seg_b);

  // deref isnt great here, and we make a bunch of copies (afaik) simply bc these operators make a ton of copies
  // (doing the math in reference would be better - this lib is a bit shite i guess :3)

  let line = glm::normalize(*seg_b - *seg_a);
  // get dist from point to either endpoint
  let dist_to_end = *seg_b - *point;
  let proj_len = glm::dot(dist_to_end, line);
  // project onto b-a  to get dist, subtract from dist to get perp component
  let perp_dist = dist_to_end - (line * glm::dvec2(proj_len, proj_len));

  // for variable width: want to return this, as well as the indices of the neighboring points
  let closest_point = *point + perp_dist;
  if seg_rect.test(&closest_point) {
    return closest_point;
  } else {
    let dist_to_start = *seg_a - *point;
    if glm::length(dist_to_start) < glm::length(dist_to_end) {
      return *seg_a;
    } else {
      return *seg_b;
    }
  }
}

fn dist_segments(seg_a: &DVec2, seg_b: &DVec2, point: &DVec2) -> f64 {
  let min_point = closest_point(seg_a, seg_b, point);
  return glm::length(*point - min_point);
}


#[derive(Clone)]
pub struct SDFLine {
  points: Vec<DVec2>
}

#[derive(Clone)]
pub struct SDFCapsule {
  path: SDFLine,
  radius: Vec<f64>
}

impl SDFCircle {
  pub fn new(x: &f64, y: &f64, radius: &f64) -> Self {
    return SDFCircle {
      point: glm::dvec2(*x, *y),
      radius: *radius
    };
  }
}

impl SDFLine {
  pub fn new(points: &Vec<DVec2>) -> Self {
    return SDFLine {
      points: points.clone()
    };
  }

  pub fn new_move(points: Vec<DVec2>) -> Self {
    return SDFLine { points };
  }
}

impl SDFCapsule {
  pub fn new(points: &Vec<DVec2>, radius: &f64) -> Self {
    let mut vec = Vec::new();
    for _ in 0..points.len() {
      vec.push(*radius);
    }

    return SDFCapsule {
      path: SDFLine::new(points),
      radius: vec
    };
  }

  pub fn new_variable(points: Vec<DVec2>, rads: Vec<f64>) -> Self {
    return SDFCapsule {
      path: SDFLine::new_move(points),
      radius: rads
    }
  }

  pub fn new_move(points: Vec<DVec2>, radius: &f64) -> Self {
    let mut vec = Vec::new();
    for _ in 0..points.len() {
      vec.push(*radius);
    }

    return SDFCapsule {
      path: SDFLine::new_move(points),
      radius: vec
    };
  }
}

// tba: generalize to a bezier??

// just to ensure its taken care of
pub trait Marchable: Send + Sync {
  /// returns the distance from the point to the sdf.
  fn dist(&self, point: &DVec2) -> f64;
}

impl Marchable for SDFCircle {
  fn dist(&self, point: &DVec2) -> f64 {
    return glm::length(self.point - *point) - self.radius;
  }
}

// smoothing: how?
// - smooth btwn batches

impl Marchable for SDFLine {
  fn dist(&self, point: &DVec2) -> f64 {
    let mut min_dist = f64::MAX;
    for i in 1..self.points.len() {
      let seg_a = self.points.get(i - 1).unwrap();
      let seg_b = self.points.get(i).unwrap();

      min_dist = f64::min(min_dist, dist_segments(seg_a, seg_b, point));
    }

    return min_dist;
  }
}

// https://www.shadertoy.com/view/4lcBWn from iq
fn dist_capsule(input_point: &DVec2, pa: DVec2, point_b: DVec2, rad_a: f64, rad_b: f64) -> f64 {
  let point  = *input_point - pa;
  let pb =     point_b - pa;
  // len-sqr
  let h = glm::dot(pb, pb);
  let mut q = dvec2(glm::dot(point, dvec2(pb.y, -pb.x)), glm::dot(point, pb)) / h;

  q.x = glm::abs(q.x);

  let b = rad_a - rad_b;
  let c = dvec2(glm::sqrt(f64::max(h - b * b, 0.00001)), b);

  let k = c.x * q.y - c.y * q.x;
  let m = glm::dot(c, q);
  let n = glm::dot(q, q);

  if k < 0.0 {
    return sqrt(h * n) - rad_a;
  } else if k > c.x {
    return sqrt(h * (n + 1.0 - 2.0 * q.y)) - rad_b;
  }
  
  return m - rad_a;
}

impl Marchable for SDFCapsule {
  fn dist(&self, point: &DVec2) -> f64 {

    let mut min_dist = f64::MAX;
    for i in 1..self.path.points.len() {
      let seg_a = self.path.points.get(i - 1).unwrap();
      let seg_b = self.path.points.get(i).unwrap();

      let rad_a = self.radius.get(i - 1).unwrap();
      let rad_b = self.radius.get(i).unwrap();

      min_dist = f64::min(min_dist, dist_capsule(
        point,
        seg_a.clone(),
        seg_b.clone(),
        *rad_a,
        *rad_b
      ))
    }

    return min_dist;
  }
}



struct Rect {
  start: DVec2,
  end: DVec2
}

impl Rect {
  fn from(a: &DVec2, b: &DVec2) -> Rect {
    let min = DVec2 {
      x: f64::min(a.x, b.x),
      y: f64::min(a.y, b.y)
    };
    
    let max = DVec2 {
      x: f64::max(a.x, b.x),
      y: f64::max(a.y, b.y)
    };

    return Rect {
      start: min, 
      end: max
    };

  }

  fn test(&self, point: &DVec2) -> bool {
    return point.x > self.start.x && point.x > self.end.x && point.y > self.start.y && point.y < self.end.y;
  }
}

// additional impl
// - smoothing func (templated)