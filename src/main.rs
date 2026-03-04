use serde::Deserialize;
use std::fs;
use std::net::Ipv6Addr;
use std::path::Path;
use toml;

#[derive(Debug, Deserialize)]
struct Config {
    decoder: String,

    ipv6: Ipv6Addr,
    port: String, // maybe not

    protocol: String,

    up: i32,
    down: i32,

    x_resolution: i32,
    y_resolution: i32,

    encoder: String,
}

fn main() {
    let config_path = Path::new("./config.toml");
    let config_string = fs::read_to_string(config_path).expect("null");

    let parsed_config: Config = toml::from_str(&config_string).expect("failed");

    println!("{:#?}", parsed_config);

    println!("hello :3");
}
