use crate::traits::{convert::Convert, unit::Unit};

pub struct GenericConverter;

impl Convert for GenericConverter {
    fn convert(&self, value: f64, from: &dyn Unit, to: &dyn Unit) -> f64 {
        let value_in_gold_standard = from.to_gold_standard(value);
        to.from_gold_standard(value_in_gold_standard)
    }
}
impl GenericConverter {
    pub fn new() -> Self {
        GenericConverter {}
    }
}
