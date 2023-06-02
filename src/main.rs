extern  crate toml;
extern  crate clap;

use clap::{Command};
use std::env;
use std::fs;
use std::path::Path;
use toml::{Value, to_string_pretty};
use toml::value::Table;

struct Config {
    endpoint: Option<String>,
    // Add other fields for additional arguments here
}

fn parse_args() -> Config {
    let matches = Command::new("Gyro")
        .bin_name("gyro")
        .arg(
            clap::arg!(--"endpoint" <"">)
                .value_parser(clap::value_parser!(String))
        )
        // Add other arguments here as needed
        .get_matches();

    Config {
        endpoint: matches.get_one::<String>("endpoint").map(|s| s.to_owned()),
        // Set other fields based on the corresponding arguments
    }
}


fn load_config(config_file: &Path) -> Value {
    let config_content = match fs::read_to_string(config_file) {
        Ok(content) => content,
        Err(e) => panic!("Failed to read config.toml: {}", e),
    };

    match config_content.parse() {
        Ok(value) => value,
        Err(e) => panic!("Failed to parse config.toml: {}", e),
    }
}

fn write_config(config_file: &Path, config: &Value) {
    match fs::write(config_file, to_string_pretty(config).unwrap()) {
        Ok(_) => println!("Updated config.toml"),
        Err(e) => panic!("Failed to update config.toml: {}", e),
    }
}


fn main() {
    // Get the HOME directory path
    let home_dir = match env::var("HOME") {
        Ok(val) => val,
        Err(_) => panic!("Failed to get HOME directory"),
    };

    // Build the path for the .gyro directory
    let gyro_dir = Path::new(&home_dir).join(".gyro");

    // Check if the .gyro directory exists, create it if it doesn't
    if !gyro_dir.exists() {
        match fs::create_dir(&gyro_dir) {
            Ok(_) => println!("Created .gyro directory"),
            Err(e) => panic!("Failed to create .gyro directory: {}", e),
        }
    }

    // Build the path for the config file
    let config_file = gyro_dir.join("config.toml");

    // Check if the config file exists, initialize with a sample config if it doesn't
    if !config_file.exists() {
        let mut config = Table::new();
        config.insert("endpoint".to_owned(), Value::String("".to_owned())); // Empty endpoint initially

        // Write the config to the config file
        write_config(&config_file, &Value::Table(config));
    }

    // Load the config from the config file
    let mut config = load_config(&config_file);

    // Parse the command-line arguments
    let args = parse_args();

    // Update the config with the parsed arguments
    if let Some(endpoint) = args.endpoint {
        config["endpoint"] = Value::String(endpoint);
    }

    // Write the updated config to the config file
    write_config(&config_file, &config);

    // Print the endpoint value from the configuration
    if let Some(endpoint) = config.get("endpoint").and_then(Value::as_str) {
        println!("Endpoint: {}", endpoint);
    }

    println!("Hello, Gyro!");
}