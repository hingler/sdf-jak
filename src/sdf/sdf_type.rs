use glm::DVec2;

#[derive(Clone, Copy)]
pub struct SDFCircle {
  point: DVec2,
  radius: f64
}


#[derive(Clone)]
pub struct SDFLine {
  points: Vec<DVec2>
}

#[derive(Clone)]
pub struct SDFCapsule {
  path: SDFLine,
  radius: f64
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
    return SDFCapsule {
      path: SDFLine::new(points),
      radius: *radius
    };
  }

  pub fn new_move(points: Vec<DVec2>, radius: &f64) -> Self {
    return SDFCapsule {
      path: SDFLine::new_move(points),
      radius: *radius
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

impl SDFLine {
  fn dist_segments(seg_a: &DVec2, seg_b: &DVec2, point: &DVec2) -> f64 {
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

    let closest_point = *point + perp_dist;
    if seg_rect.test(&closest_point) {
      return glm::length(perp_dist);
    }

    let dist_to_start = *seg_a - *point;
    return f64::min(glm::length(dist_to_start), glm::length(dist_to_end));
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

      min_dist = f64::min(min_dist, SDFLine::dist_segments(seg_a, seg_b, point));
    }

    return min_dist;
  }
}

impl Marchable for SDFCapsule {
  fn dist(&self, point: &DVec2) -> f64 {
    return self.path.dist(point) - self.radius;
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

