use serde::Deserialize;
use std::fs;
use std::process::exit;
use toml;

#[derive(Deserialize)]
pub struct Data {
    pub tracing_config: Tracing,
}

#[derive(Deserialize)]
pub struct Tracing {
    #[serde(deserialize_with = "deserialize_tracing_level")]
    pub level: tracing::Level,
}


fn deserialize_tracing_level<'de, D>(deserializer: D) -> Result<tracing::Level, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let level_str = String::deserialize(deserializer)?;
    let level = match level_str.to_ascii_lowercase().as_str() {
        "error" => tracing::Level::ERROR,
        "warn" => tracing::Level::WARN,
        "info" => tracing::Level::INFO,
        "debug" => tracing::Level::DEBUG,
        "trace" => tracing::Level::TRACE,
        _ => return Err(serde::de::Error::custom("Invalid tracing level")),
    };
    Ok(level)
}

pub fn read_toml_config(filename: &str) -> Data {
    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Could not read file `{}`: {}", filename, e);
            exit(1);
        }
    };

    eprintln!("Loading contents `{}`", contents);
    match toml::from_str(&contents) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Unable to load data from `{}`: {}", filename, e);
            exit(1);
        }
    }
}
