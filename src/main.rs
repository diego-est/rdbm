mod dbm;
use dbm::handle_query;
use std::{env, path::PathBuf};
use xdg_home::home_dir;

#[warn(clippy::pedantic, clippy::unwrap_used, clippy::perf)]
fn main() {
    let config_path: PathBuf = match env::var("XDG_CONFIG_HOME") {
        Ok(path) => path.into(),
        Err(_) => match home_dir() {
            Some(path) => path,
            None => "".into(),
        },
    }
    .join("resources.ron");

    handle_query(config_path);
}
