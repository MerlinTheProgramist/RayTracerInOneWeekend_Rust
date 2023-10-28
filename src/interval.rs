use crate::Num;

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
    pub fn new(_min: Num, _max: Num) -> Interval {
        Interval {
            min: _min,
            max: _max,
        }
    }
    pub fn contains(&self, x: Num) -> bool {
        self.min <= x && x <= self.max
    }
    pub fn surrounds(&self, x: Num) -> bool {
        self.min < x && x < self.max
    }

    pub const EMPTY: Interval = Interval {
        min: Num::MAX,
        max: Num::MIN,
    };
    pub const UNIVERSE: Interval = Interval {
        min: Num::MIN,
        max: Num::MAX,
    };
}
