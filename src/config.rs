use niri_ipc::Window;
use serde::Deserialize;
use std::{env, fs, path::Path};

use crate::window_rules::WindowRule;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    #[serde(rename = "window-rule")]
    window_rules: Vec<WindowRule>,
}

impl Config {
    pub fn evaluate_window(&mut self, window: &Window) {
        self.window_rules.retain_mut(|rule| {
            rule.evaluate(window);
            !rule.exhausted()
        })
    }

    pub fn add_window_rule(&mut self, window_rule: WindowRule) {
        self.window_rules.push(window_rule);
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

    if let Ok(content) = fs::read_to_string(config_path) {
        toml::from_str(&content).expect("Failed to parse config file")
    } else {
        Config {
            window_rules: vec![],
        }
    }
}
