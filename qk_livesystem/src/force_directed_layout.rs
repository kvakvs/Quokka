use crate::ui::layout::Layout;
use crate::ui::point::{Pointf, FastSqrt};

/// Implements force directed layout attactions and repulsions according to:
/// https://cs.brown.edu/people/rtamassi/gdhandbook/chapters/force-directed.pdf
pub struct Force {
  pub p: Pointf
}

impl Into<Pointf> for Force {
  fn into(self) -> Pointf {
    self.p
  }
}

impl Force {
  pub fn new() -> Self {
    Force {
      p: Pointf::default()
    }
  }

  /// Attraction is d^2 / k, but we also normalize the distance so dir * d / strength
  pub fn add_attraction_force(&mut self,
                              point: &Pointf, other_point: &Pointf,
                              strength: f32) {
    let dir = *point - *other_point; // flipped args
    let squared_dist = dir.squared_magnitude();

    // Postpone dist calculation till we know its non-zero and safe to divide
    if squared_dist.abs() < 0.00001 { return; }

    let dist = FastSqrt::inv_sqrt(squared_dist);

    // Attraction force will be dist^2/strength
    self.p += dir / (dist * strength);
    // println!("Dbg dir={:?} sqd={:?} d={:?} prev={:?} self.p={:?}", dir, squared_dist, dist, prev_p, self.p);
  }

  // Repulsion force is -strength^2 / dist
  pub fn add_repulsion_force(&mut self,
                             point: &Pointf, other_point: &Pointf,
                             strength: f32) {
    let dir = *other_point - *point;
    let squared_dist = dir.squared_magnitude();

    // Postpone dist calculation till we know its non-zero and safe to divide
    if squared_dist.abs() < 0.00001 { return; }
    // let dist = FastSqrt::inv_sqrt(squared_dist);

    // Divide by squared dist because one dist is to normalize, other to implement -k^2/d formula
    let prev_p = self.p;
    self.p += dir * (strength * strength / squared_dist);
    println!("Dbg repulsion: dir={:?} sqd={:?} prev={:?} self.p={:?}", dir, squared_dist, prev_p, self.p);
  }
}
