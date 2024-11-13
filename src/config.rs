use niri_ipc::Window;
use serde::Deserialize;
use std::{env, fs, path::Path};

use crate::window_rules::WindowRule;

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    #[serde(rename = "window-rule")]
    window_rules: Vec<WindowRule>,
}

impl Config {
    pub fn evaluate(&self, window: &Window) {
        for rule in self.window_rules.iter() {
            rule.evaluate(window);
        }
    }
}

pub fn read_config() -> Config {
    let config_dir = env::var("XDG_CONFIG_HOME").map_or_else(
        |_| {
            let home_path = env::var("HOME").expect("Neither $HOME nor $XDG_CONFIG_HOME set");
            Path::new(&home_path).join(".config")
        },
        |path| Path::new(&path).to_path_buf(),
    );
    let config_path = config_dir.join("niri-helper.toml");

    let content = fs::read_to_string(config_path).expect("Failed to read config file");
    toml::from_str(&content).expect("Failed to parse config file")
}
