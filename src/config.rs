use std::{env, fs, path::Path};

use niri_ipc::{Action, Request, Window};

use crate::ipc::send_command;

pub type WindowRules = Vec<WindowRule>;

pub type WindowRule = (WindowMatcher, WindowAction);

pub enum WindowMatcher {
    Title(String),
    AppId(String),
}

pub enum WindowAction {
    Column(u64),
    InCurrentColumn(bool),
    InColumn(u64),
}

impl WindowMatcher {
    pub fn new(match_type: &str, match_value: &str) -> WindowMatcher {
        match match_type {
            "title" => WindowMatcher::Title(match_value.to_owned()),
            "app_id" => WindowMatcher::AppId(match_value.to_owned()),
            _ => panic!("Invalid match_type"),
        }
    }

    pub fn match_window(&self, window: &Window) -> bool {
        match self {
            WindowMatcher::Title(action_title) => {
                matches!(window, Window { title: Some(window_title), .. } if window_title == action_title)
            }
            WindowMatcher::AppId(action_app_id) => {
                matches!(window, Window { app_id: Some(window_app_id), .. } if window_app_id == action_app_id)
            }
        }
    }
}

impl WindowAction {
    pub fn perform(&self, id: u64) {
        let id = Some(id);

        match self {
            WindowAction::Column(column) => {
                WindowAction::move_focused_to_column(column);
            }
            WindowAction::InCurrentColumn(in_current_column) if *in_current_column => {
                send_command(Request::Action(Action::ConsumeOrExpelWindowLeft { id }));
            }
            WindowAction::InColumn(column) => {
                WindowAction::move_focused_to_column(column);
                send_command(Request::Action(Action::ConsumeOrExpelWindowRight { id }));
            }
            _ => (),
        };
    }

    fn move_focused_to_column(column: &u64) {
        send_command(Request::Action(Action::MoveColumnToFirst {}));
        for _ in 1..column.to_owned() {
            send_command(Request::Action(Action::MoveColumnRight {}));
        }
    }
}

pub fn read_config() -> WindowRules {
    let home_path = env::var("HOME").expect("$HOME not set");
    let config_path = Path::new(&home_path)
        .join(".config")
        .join("niri-helper.toml");

    let content = fs::read_to_string(config_path).expect("Failed to read config file");
    let config_toml = content
        .parse::<toml::Value>()
        .expect("Failed to parse toml file");

    parse_config(config_toml)
}

fn parse_config(config_toml: toml::Value) -> WindowRules {
    let mut config = Vec::new();

    match config_toml {
        toml::Value::Table(table) => {
            for (match_type, v) in table.iter() {
                match v {
                    toml::Value::Table(table) => {
                        for (match_value, v) in table.iter() {
                            let matcher = WindowMatcher::new(match_type, match_value);
                            let action = parse_action(v);
                            config.push((matcher, action));
                        }
                    }
                    _ => panic!("Invalid configuration"),
                }
            }
        }
        _ => panic!("Invalid configuration"),
    };

    config
}

fn parse_action(rule: &toml::Value) -> WindowAction {
    if let toml::Value::Table(table) = rule {
        if let Some((action, v)) = table.iter().next() {
            match action.as_str() {
                "column" => {
                    if let toml::Value::Integer(n) = v {
                        WindowAction::Column(n.to_owned() as u64)
                    } else {
                        panic!("Invalid configuration");
                    }
                }
                "in_current_column" => {
                    if let toml::Value::Boolean(b) = v {
                        WindowAction::InCurrentColumn(*b)
                    } else {
                        panic!("Invalid configuration");
                    }
                }
                "in_column" => {
                    if let toml::Value::Integer(n) = v {
                        WindowAction::InColumn(n.to_owned() as u64)
                    } else {
                        panic!("Invalid configuration");
                    }
                }
                _ => panic!("Invalid configuration"),
            }
        } else {
            panic!("Invalid configuration");
        }
    } else {
        panic!("Invalid configuration");
    }
}
