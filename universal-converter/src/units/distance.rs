use crate::traits::unit::Unit;
#[derive(Clone)]
pub enum Distance {
    Kilometer(f64),
    Meter(f64),
    Centimeter(f64),
    Millimeter(f64),
    Mile(f64),
    Yard(f64),
    Foot(f64),
    Inch(f64),
}

impl Unit for Distance {
    fn to_gold_standard(&self, value: f64) -> f64 {
        match self {
            Distance::Kilometer(multiplier) => value * multiplier,
            Distance::Meter(multiplier) => value * multiplier,
            Distance::Centimeter(multiplier) => value * multiplier,
            Distance::Millimeter(multiplier) => value * multiplier,
            Distance::Mile(multiplier) => value * multiplier,
            Distance::Yard(multiplier) => value * multiplier,
            Distance::Foot(multiplier) => value * multiplier,
            Distance::Inch(multiplier) => value * multiplier,
        }
    }

    fn from_gold_standard(&self, value: f64) -> f64 {
        match self {
            Distance::Kilometer(multiplier) => value / multiplier,
            Distance::Meter(multiplier) => value / multiplier,
            Distance::Centimeter(multiplier) => value / multiplier,
            Distance::Millimeter(multiplier) => value / multiplier,
            Distance::Mile(multiplier) => value / multiplier,
            Distance::Yard(multiplier) => value / multiplier,
            Distance::Foot(multiplier) => value / multiplier,
            Distance::Inch(multiplier) => value / multiplier,
        }
    }
}
