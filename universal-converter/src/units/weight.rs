use crate::traits::unit::Unit;

#[derive(Clone)]
pub enum Weight {
    Kilogram(f64), // 1 kg = 1000 g
    Gram(f64),
    Milligram(f64), // 1 mg = 0.001 g
    Pound(f64),     // 1 lb = 453.59237 g
    Ounce(f64),     // 1 oz = 28.3495231 g
}

impl Weight {
    pub fn name(&self) -> &str {
        match self {
            Weight::Kilogram(_) => "kg",
            Weight::Gram(_) => "g",
            Weight::Milligram(_) => "mg",
            Weight::Pound(_) => "lb",
            Weight::Ounce(_) => "oz",
        }
    }
}

impl Unit for Weight {
    fn to_gold_standard(&self, value: f64) -> f64 {
        match self {
            Weight::Kilogram(multiplier) => value * multiplier,
            Weight::Gram(multiplier) => value * multiplier,
            Weight::Milligram(multiplier) => value * multiplier,
            Weight::Pound(multiplier) => value * multiplier,
            Weight::Ounce(multiplier) => value * multiplier,
        }
    }

    fn from_gold_standard(&self, value: f64) -> f64 {
        match self {
            Weight::Kilogram(multiplier) => value / multiplier,
            Weight::Gram(multiplier) => value / multiplier,
            Weight::Milligram(multiplier) => value / multiplier,
            Weight::Pound(multiplier) => value / multiplier,
            Weight::Ounce(multiplier) => value / multiplier,
        }
    }
}
