use crate::currency::currency_helper::get_exchange_rates;
use crate::units::measurement::Measurement;
use clap::Parser;
use std::collections::HashMap;
use std::env;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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
    Currency,
    Time,
}
impl ConversionType {
    pub fn get_hashmap(&self) -> HashMap<String, Measurement> {
        let mut measurements: HashMap<String, Measurement> = HashMap::new();
        match self {
            ConversionType::Distance => {
                measurements.insert("km".to_string(), Measurement::new(1.0 / 1000.0)); // 1 km = 1000 m
                measurements.insert("m".to_string(), Measurement::new(1.0)); // 1 m = 1 m
                measurements.insert("cm".to_string(), Measurement::new(1.0 / 0.01)); // 1 cm = 0.01 m
                measurements.insert("mm".to_string(), Measurement::new(1.0 / 0.001)); // 1 mm = 0.001 m
                measurements.insert("mi".to_string(), Measurement::new(1.0 / 1609.344)); // 1 mi = 1609.344 m
                measurements.insert("yd".to_string(), Measurement::new(1.0 / 0.9144)); // 1 yd = 0.9144 m
                measurements.insert("ft".to_string(), Measurement::new(1.0 / 0.3048)); // 1 ft = 0.3048 m
                measurements.insert("in".to_string(), Measurement::new(1.0 / 0.0254));
                // 1 in = 0.0254 m
            }
            ConversionType::Weight => {
                measurements.insert("kg".to_string(), Measurement::new(1.0)); // 1 kg = 1 kg
                measurements.insert("g".to_string(), Measurement::new(1.0 / 0.001)); // 1 g = 0.001 kg
                measurements.insert("mg".to_string(), Measurement::new(1.0 / 0.000001)); // 1 mg = 0.000001 kg
                measurements.insert("lb".to_string(), Measurement::new(1.0 / 0.45359237)); // 1 lb = 0.45359237 kg
                measurements.insert("oz".to_string(), Measurement::new(1.0 / 0.0283495));
                // 1 oz = 0.0283495 kg
            }
            ConversionType::Time => {
                measurements.insert("s".to_string(), Measurement::new(1.0)); // 1 s = 1 s
                measurements.insert("min".to_string(), Measurement::new(1.0 / 60.0)); // 1 min = 60 s
                measurements.insert("h".to_string(), Measurement::new(1.0 / 3600.0)); // 1 h = 3600 s
                measurements.insert("d".to_string(), Measurement::new(1.0 / 86400.0)); // 1 d = 86400 s
                measurements.insert("w".to_string(), Measurement::new(1.0 / 604800.0));
                // 1 w = 604800 s
            }
            ConversionType::Currency => {
                let get_env = |key: &str| {
                    env::var(key).expect(&format!("Environment variable {} not set", key))
                };
                let max_currency_data_age: u64 = get_env("MAX_CURRENCY_DATA_AGE")
                    .parse()
                    .expect("MAX_CURRENCY_DATA_AGE must be an integer");

                let json_path = get_env("CURRENCIES_JSON");
                let api_url = get_env("API_URL");
                let api_key = get_env("API_KEY");

                let exchange_rates =
                    get_exchange_rates(&json_path, max_currency_data_age, &api_url, &api_key);

                measurements = exchange_rates; // Assign the fetched exchange rates to `measurements`
            }
        }
        measurements
    }
    pub fn infer_conversion_type(from_unit: &str, to_unit: &str) -> Option<ConversionType> {
        let mut found_type: Option<ConversionType> = None;
        let mut matching_hashmaps: Vec<HashMap<String, Measurement>> = Vec::new();

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
