use std::fs::{read_to_string, OpenOptions};
use std::io::Write;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct Settings {
    setting: Vec<Setting>
}

#[derive(Debug, Serialize, Deserialize)]
struct Setting {
    os: String,
    version: f32,
    users: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let setting = Setting {
        os: "macOS".into(),
        version: 11.5,
        users: vec!["Bob".into(), "Alice".into()],
    };

    let toml = toml::to_string(&setting).unwrap();
    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open("Setting.toml")?;
    write!(f, "[[setting]]\n{}\n", toml)?;
    f.flush()?;
    println!("\nTOML:\n{}", toml);

    let foo: String = read_to_string("Setting.toml")?;
    let setting: Result<Settings, toml::de::Error> = toml::from_str(&foo);
    match setting {
        Ok(p) => println!("#Parsed TOML:\n{:#?}", p),
        Err(e) => panic!("Filed to parse TOML: {}", e),
    }

    Ok(())
}
