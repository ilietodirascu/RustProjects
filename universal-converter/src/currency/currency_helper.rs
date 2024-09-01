use crate::units::measurement::Measurement;
use reqwest::blocking;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::{Duration, SystemTime};

fn fetch_json_from_api(api_url: &str, api_key: &str) -> Value {
    // Construct the full URL with the API key
    let full_url = format!("{}?access_key={}", api_url, api_key);

    // Perform the API request
    let response = blocking::get(&full_url).expect("Failed to make API request");

    // Parse and return the JSON response
    let json = response
        .json::<Value>()
        .expect("Failed to parse JSON response");
    json
}

fn save_json_to_file(file_path: &str, json: &Value) {
    let json_string = serde_json::to_string_pretty(json).expect("Failed to serialize JSON");
    fs::write(file_path, json_string).expect("Failed to write file");
}

fn file_exists(file_path: &str) -> bool {
    Path::new(file_path).exists()
}

fn is_file_empty(file_path: &str) -> bool {
    let metadata = fs::metadata(file_path).expect("Unable to read metadata");
    metadata.len() == 0
}

fn is_file_recent(file_path: &str, max_age: Duration) -> bool {
    let metadata = fs::metadata(file_path).expect("Unable to read metadata");
    let modified_time = metadata.modified().expect("Unable to get modified time");
    let current_time = SystemTime::now();
    let age = current_time
        .duration_since(modified_time)
        .expect("Time went backwards");
    age <= max_age
}

fn load_json(file_path: &str) -> Value {
    let file_content = fs::read_to_string(file_path).expect("Unable to read file");
    serde_json::from_str(&file_content).expect("Unable to parse JSON")
}

fn extract_exchange_rates(json: &Value) -> HashMap<String, Measurement> {
    let rates = json["rates"]
        .as_object()
        .expect("Expected rates to be an object");
    let mut exchange_rates: HashMap<String, Measurement> = HashMap::new();

    for (key, value) in rates.iter() {
        let rate = value.as_f64().expect("Expected rate to be a float");
        exchange_rates.insert(key.clone(), Measurement::new(rate));
    }

    exchange_rates
}

pub fn get_exchange_rates(
    file_path: &str,
    max_data_age: u64,
    api_url: &str,
    api_key: &str,
) -> HashMap<String, Measurement> {
    // Check if the file exists
    let file_exists = file_exists(file_path);

    // Fetch new data if the file doesn't exist, is empty, or is too old
    if !file_exists
        || is_file_empty(file_path)
        || !is_file_recent(file_path, Duration::from_secs(max_data_age))
    {
        eprintln!("Fetching new data from API...");

        // Fetch JSON data from API
        let json = fetch_json_from_api(api_url, api_key);

        // Save the fetched JSON data to the file
        save_json_to_file(file_path, &json);
    }

    // Load the JSON and extract exchange rates
    let json = load_json(file_path);
    extract_exchange_rates(&json)
}
