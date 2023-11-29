mod args;
use args::{CLIArgs, Command};
use clap::Parser;
use ron::error::Error;
use ron::error::SpannedError;
use ron::ser;
use ron::ser::PrettyConfig;
use std::{collections::HashMap, fs, io, path::PathBuf};

/* === Types === */
enum QueryError {
    ParseError(String),
    FileMissing(PathBuf),
    KeyMissing(String),
    IOError(io::Error),
}

impl From<SpannedError> for QueryError {
    fn from(e: SpannedError) -> Self {
        QueryError::ParseError(format!(
            "Parse error at position: {}. In code: {}",
            e.position, e.code
        ))
    }
}

impl From<Error> for QueryError {
    fn from(e: Error) -> Self {
        QueryError::ParseError(format!(
            "Parse error: {}",
            match e {
                Error::Message(string) => string,
                _ => "".to_string(),
            }
        ))
    }
}

impl QueryError {
    fn display(&self) {
        match self {
            QueryError::ParseError(string) => eprintln!("{string}"),
            QueryError::FileMissing(path) => eprintln!("File missing: {}", path.display()),
            QueryError::IOError(err) => eprintln!("{err}"),
            QueryError::KeyMissing(key) => eprintln!("The following key was not found: {key}"),
        }
    }
}

type RonMap = HashMap<String, String>;

/* === Functions === */

#[warn(clippy::pedantic, clippy::unwrap_used, clippy::perf)]
pub fn handle_query(config: PathBuf) {
    match check_config(config.clone()) {
        Ok(ron_map) => match query_resources(ron_map, config) {
            Ok(string) => println!("{string}"),
            Err(e) => e.display(),
        },
        Err(e) => e.display(),
    }
}

#[warn(clippy::pedantic, clippy::unwrap_used, clippy::perf)]
fn check_config(config: PathBuf) -> Result<RonMap, QueryError> {
    match config.try_exists() {
        Ok(exists) => {
            if exists {
                let contents =
                    fs::read_to_string(config).expect("Query exists and has proper permissions.");
                Ok(ron::from_str(&contents)?)
            } else {
                Err(QueryError::FileMissing(config))
            }
        }
        Err(e) => Err(QueryError::IOError(e)),
    }
}

#[warn(clippy::pedantic, clippy::unwrap_used, clippy::perf)]
fn query_resources(mut map: RonMap, config: PathBuf) -> Result<String, QueryError> {
    let args = CLIArgs::parse();

    match args.command {
        Command::All => Ok(ser::to_string_pretty(&map, PrettyConfig::default())?),
        Command::Get(key) => match map.get_key_value(&key.key) {
            Some(entry) => Ok(entry.1.to_string()),
            None => Err(QueryError::KeyMissing(key.key)),
        },
        Command::Set(key_val) => {
            let key = key_val.key;
            let value = key_val.value;
            map.insert(key.clone(), value.clone());
            let pretty = ser::to_string_pretty(&map, PrettyConfig::default())?;

            match fs::write(config, pretty) {
                Ok(()) => Ok(format!("\"{key}\": \"{value}\"")),
                Err(e) => Err(QueryError::IOError(e)),
            }
        }
    }
}
