use daemon::Daemon;

mod config;
mod daemon;
mod ipc;
mod niri_tracker;
mod rules_common;
mod window_rules;

fn main() {
    let daemon = Daemon::new();
    daemon.events_listen();
}
