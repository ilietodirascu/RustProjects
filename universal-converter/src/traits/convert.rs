use super::unit::Unit;
pub trait Convert {
    fn convert(&self, value: f64, from: &dyn Unit, to: &dyn Unit) -> f64;
}
