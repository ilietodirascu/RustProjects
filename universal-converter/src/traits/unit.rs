pub trait Unit {
    fn to_gold_standard(&self, value: f64) -> f64;
    fn from_gold_standard(&self, value: f64) -> f64;
}
