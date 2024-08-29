use clap::Parser;
use universal_converter::{
    args::{Args, ConversionType},
    converters::generic::GenericConverter,
    traits::convert::Convert,
};

fn main() {
    let args = Args::parse();

    let Some(conversion_type) = args.conversion_type.or_else(|| {
        // Attempt to infer the conversion type
        ConversionType::infer_conversion_type(&args.from_unit, &args.to_unit)
    }) else {
        eprintln!("Unsupported conversion or ambiguous conversion type.");
        return;
    };

    let conversions_hashmap = conversion_type.get_hashmap();

    let from_unit_str = args.from_unit.as_str();
    let to_unit_str = args.to_unit.as_str();

    let from_unit = conversions_hashmap.get(from_unit_str).unwrap_or_else(|| {
        eprintln!("Unit '{}' not supported", from_unit_str);
        std::process::exit(1);
    });
    let to_unit = conversions_hashmap.get(to_unit_str).unwrap_or_else(|| {
        eprintln!("Unit '{}' not supported", to_unit_str);
        std::process::exit(1);
    });
    let converter = GenericConverter::new();

    for value in args.values.iter() {
        let result = converter.convert(*value, &**from_unit, &**to_unit);
        let formatted_value = format!("{:.4}", value);
        let formatted_result = format!("{:.4}", result);

        println!(
            "{} {} is {} {}",
            formatted_value, from_unit_str, formatted_result, to_unit_str
        );
    }
}
