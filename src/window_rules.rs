use niri_ipc::{Action, Request, SizeChange, Window};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{niri_ipc::send_command, process::is_child_process, rules_common::RuleLifetime};

#[derive(Deserialize, Serialize, Default, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct WindowRule {
    #[serde(default)]
    pub app_id: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub pid: Option<u32>,
    #[serde(default)]
    pub match_strategy: MatchStrategy,

    #[serde(default)]
    pub column: Option<u64>,
    #[serde(default)]
    pub in_current_column: Option<bool>,
    #[serde(default)]
    pub in_column: Option<u64>,
    #[serde(default)]
    pub close: Option<bool>,
    #[serde(default)]
    pub fixed_width: Option<i32>,

    #[serde(default)]
    pub rule_lifetime: Option<RuleLifetime>,
}

#[derive(Eq, PartialEq, Deserialize, Serialize, Debug)]
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
    pub fn evaluate(&mut self, window: &Window) {
        if self.match_window(window) {
            if !self.exhausted() {
                self.update_rule_lifetime();
                self.perform(window);
            }
        }
    }

    pub fn exhausted(&self) -> bool {
        matches!(self.rule_lifetime, Some(RuleLifetime::Matches(0)))
    }

    fn update_rule_lifetime(&mut self) {
        if let Some(RuleLifetime::Matches(matches)) = self.rule_lifetime {
            let new_matches = (matches - 1).clamp(0, u64::MAX);
            self.rule_lifetime = Some(RuleLifetime::Matches(new_matches));
        }
    }

    fn match_window(&self, window: &Window) -> bool {
        let app_id_match = WindowRule::match_property(self.app_id.clone(), window.app_id.clone());
        let title_match = WindowRule::match_property(self.title.clone(), window.title.clone());

        let pid_match = match (self.pid, window.pid) {
            (Some(parent_pid), Some(child_pid)) => is_child_process(child_pid as u32, parent_pid),
            _ => false,
        };

        if self.match_strategy == MatchStrategy::All {
            app_id_match && title_match && pid_match
        } else {
            app_id_match || title_match || pid_match
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
        if let Some(true) = self.close {
            send_command(Request::Action(Action::CloseWindow { id }));
        }
        if let Some(width) = self.fixed_width {
            send_command(Request::Action(Action::SetColumnWidth {
                change: SizeChange::SetFixed(width),
            }));
        }
    }

    fn move_focused_to_column(column: &u64) {
        send_command(Request::Action(Action::MoveColumnToFirst {}));
        for _ in 1..column.to_owned() {
            send_command(Request::Action(Action::MoveColumnRight {}));
        }
    }
}
