use crate::Num;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Interval {
    pub min: Num,
    pub max: Num,
}

impl Interval {
    pub fn default() -> Interval {
        Interval {
            min: Num::MIN,
            max: Num::MAX,
        }
    }
    pub fn new(min: Num, max: Num) -> Interval {
        Interval { min, max }
    }

    /// return Interval that contains both intervals
    pub fn union(a: &Self, b: &Self) -> Self {
        Self::new(a.min.min(b.min), a.max.max(b.max))
    }

    /// Check if interval contains value
    pub fn contains(&self, x: Num) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: Num) -> bool {
        self.min < x && x < self.max
    }
    pub fn clamp(&self, x: Num) -> Num {
        Num::min(Num::max(x, self.min), self.max)
    }
    pub fn size(&self) -> Num {
        self.max - self.min
    }
    pub fn expand(&self, delta: Num) -> Self {
        let padding = delta / 2.0;
        Self::new(self.min - padding, self.max + padding)
    }

    pub const EMPTY: Interval = Interval { min: 0.0, max: 0.0 };
    pub const UNIVERSE: Interval = Interval {
        min: Num::MIN,
        max: Num::MAX,
    };
}
