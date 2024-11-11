use niri_ipc::{Event, Window};

mod config;
mod ipc;

use config::{read_config, WindowRules};

fn main() {
    let window_rules = read_config();
    let mut event_reader = ipc::get_event_reader();

    let mut known_window_ids: Vec<u64> = Vec::new();

    loop {
        let event = event_reader();
        handle_event(&event, &window_rules, &mut known_window_ids);
    }
}

fn handle_event(event: &Event, window_rules: &WindowRules, known_window_ids: &mut Vec<u64>) {
    match event {
        Event::WindowsChanged { windows } => update_known_window_ids(known_window_ids, windows),
        Event::WindowClosed { id } => {
            known_window_ids.retain(|x| x != id);
        }
        Event::WindowOpenedOrChanged { window } => {
            handle_window_opened_or_changed(window, window_rules, known_window_ids);
        }
        _ => (),
    }
}

fn update_known_window_ids(known_window_ids: &mut Vec<u64>, windows: &Vec<Window>) {
    known_window_ids.clear();
    for window in windows {
        known_window_ids.push(window.id);
    }
}

fn handle_window_opened_or_changed(
    window: &Window,
    window_rules: &WindowRules,
    known_window_ids: &mut Vec<u64>,
) {
    if !known_window_ids.contains(&window.id) {
        known_window_ids.push(window.id);
        window_rules.evaluate(window);
    }
}
