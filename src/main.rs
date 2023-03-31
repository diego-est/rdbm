mod args;
use args::{EntityType, ResArgs};
use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::process::exit;

#[warn(clippy::pedantic, clippy::unwrap_used, clippy::perf)]
fn main() {
    const CONFIG_LOCATION: &str = "/home/sol/.config/resources.ron";
    let Ok(contents) = fs::read_to_string(CONFIG_LOCATION) else {
        eprintln!("Could not read config at `{CONFIG_LOCATION}`.");
        exit(1)
    };

    let mut data: HashMap<String, String> = if let Ok(map) = ron::from_str(&contents) {
        map
    } else {
        eprintln!("Could not parse config at `{CONFIG_LOCATION}`.");
        exit(1)
    };

    let args = ResArgs::parse();

    match args.option {
        EntityType::All => {
            println!("{data:#?}");
        }

        EntityType::Get(user_key) => {
            let value = if let Some(entry) = data.get_key_value(&user_key.key) {
                entry.1
            } else {
                eprintln!("Could not find `{}` in config file.", user_key.key);
                exit(1)
            };
            print!("{value}");
        }

        EntityType::Set(user_keyval) => {
            let key = user_keyval.key;
            let value = user_keyval.value;
            data.insert(key.clone(), value.clone());
            let Ok(pretty) = ron::ser::to_string_pretty(&data, ron::ser::PrettyConfig::default()) else {
                eprintln!("Could not parse map to string.");
                exit(1)
            };

            if let Ok(()) = fs::write(CONFIG_LOCATION, pretty.clone()) {
                println!("\"{}\": \"{}\"", key, value);
            } else {
                eprintln!("Error writing file.");
                exit(1)
            };
        }
    }
}
