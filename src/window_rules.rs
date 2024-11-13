use niri_ipc::{Action, Request, Window};
use regex::Regex;
use serde::Deserialize;

use crate::ipc::send_command;

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct WindowRule {
    #[serde(default)]
    app_id: Option<String>,
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    match_strategy: MatchStrategy,

    #[serde(default)]
    column: Option<u64>,
    #[serde(default)]
    in_current_column: Option<bool>,
    #[serde(default)]
    in_column: Option<u64>,
}

#[derive(Eq, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MatchStrategy {
    Any,
    All,
}

impl Default for MatchStrategy {
    fn default() -> Self {
        MatchStrategy::Any
    }
}

impl WindowRule {
    pub fn evaluate(&self, window: &Window) {
        if self.match_window(window) {
            self.perform(window);
        }
    }

    fn match_window(&self, window: &Window) -> bool {
        let app_id_match = WindowRule::match_property(self.app_id.clone(), window.app_id.clone());
        let title_match = WindowRule::match_property(self.title.clone(), window.title.clone());

        if self.match_strategy == MatchStrategy::All {
            app_id_match && title_match
        } else {
            app_id_match || title_match
        }
    }

    fn match_property(property_regex: Option<String>, window_property: Option<String>) -> bool {
        property_regex
            .zip(window_property)
            .map_or(false, |(regex, window_property)| {
                Regex::new(&regex)
                    .expect("Invalid regex")
                    .is_match(&window_property)
            })
    }

    fn perform(&self, window: &Window) {
        let id = Some(window.id);

        if let Some(column) = self.column {
            WindowRule::move_focused_to_column(&column);
        }
        if let Some(column) = self.in_column {
            WindowRule::move_focused_to_column(&column);
            send_command(Request::Action(Action::ConsumeOrExpelWindowRight { id }));
        }
        if let Some(true) = self.in_current_column {
            send_command(Request::Action(Action::ConsumeOrExpelWindowLeft { id }));
        }
    }

    fn move_focused_to_column(column: &u64) {
        send_command(Request::Action(Action::MoveColumnToFirst {}));
        for _ in 1..column.to_owned() {
            send_command(Request::Action(Action::MoveColumnRight {}));
        }
    }
}
