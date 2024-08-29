use clap::Parser;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{
    traits::unit::Unit,
    units::{distance::Distance, weight::Weight},
};

#[derive(Parser, Debug)]
#[command(
    name = "convert",
    version = "0.1.0",
    author = "Ilie",
    about = "A simple CLI tool to convert units"
)]
pub struct Args {
    /// Sets the conversion type (temperature)
    #[arg(short = 't', long = "conversion-type", value_enum)]
    pub conversion_type: Option<ConversionType>,
    #[arg()]
    pub from_unit: String,

    #[arg()]
    pub to_unit: String,

    pub values: Vec<f64>,
}

#[derive(clap::ValueEnum, Clone, Debug, EnumIter)]
pub enum ConversionType {
    Distance,
    Weight,
}
impl ConversionType {
    pub fn get_hashmap(&self) -> HashMap<String, Box<dyn Unit>> {
        let mut measurements: HashMap<String, Box<dyn Unit>> = HashMap::new();
        match self {
            ConversionType::Distance => {
                measurements.insert("km".to_string(), Box::new(Distance::Kilometer(1000.0)));
                measurements.insert("m".to_string(), Box::new(Distance::Meter(1.0)));
                measurements.insert("cm".to_string(), Box::new(Distance::Centimeter(0.01)));
                measurements.insert("mm".to_string(), Box::new(Distance::Millimeter(0.001)));
                measurements.insert("mi".to_string(), Box::new(Distance::Mile(1609.344)));
                measurements.insert("yd".to_string(), Box::new(Distance::Yard(0.9144)));
                measurements.insert("ft".to_string(), Box::new(Distance::Foot(0.3048)));
                measurements.insert("in".to_string(), Box::new(Distance::Inch(0.0254)));
            }
            ConversionType::Weight => {
                measurements.insert("kg".to_string(), Box::new(Weight::Kilogram(1000.0)));
                measurements.insert("g".to_string(), Box::new(Weight::Gram(1.0)));
                measurements.insert("mg".to_string(), Box::new(Weight::Milligram(0.001)));
                measurements.insert("lb".to_string(), Box::new(Weight::Pound(453.59237)));
                measurements.insert("oz".to_string(), Box::new(Weight::Ounce(28.3495)));
            }
        }
        measurements
    }
    pub fn infer_conversion_type(from_unit: &str, to_unit: &str) -> Option<ConversionType> {
        let mut found_type: Option<ConversionType> = None;
        let mut matching_hashmaps: Vec<HashMap<String, Box<dyn Unit>>> = Vec::new();

        for conversion_type in ConversionType::iter() {
            let hashmap = conversion_type.get_hashmap();
            if hashmap.contains_key(from_unit) && hashmap.contains_key(to_unit) {
                if found_type.is_some() {
                    // More than one match found
                    return None;
                }
                found_type = Some(conversion_type);
                matching_hashmaps.push(hashmap);
            }
        }
        if matching_hashmaps.len() > 1 {
            found_type = None;
        }

        found_type
    }
}
