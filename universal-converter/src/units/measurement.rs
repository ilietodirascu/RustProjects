use crate::traits::unit::Unit;

pub struct Measurement {
    multiplier: f64,
}

impl Measurement {
    pub fn new(multiplier: f64) -> Self {
        Measurement { multiplier }
    }
}

impl Unit for Measurement {
    fn to_gold_standard(&self, value: f64) -> f64 {
        value / self.multiplier
    }

    fn from_gold_standard(&self, value: f64) -> f64 {
        value * self.multiplier
    }
}
