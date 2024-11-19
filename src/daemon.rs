use std::sync::{Arc, Mutex, MutexGuard};

use niri_ipc::{Event, Window};

use crate::{
    config::{read_config, Config},
    ipc::Ipc,
    niri_ipc::get_event_reader,
    niri_tracker::NiriTracker,
};

pub struct Daemon {
    config: Arc<Mutex<Config>>,
    tracker: NiriTracker,
}

impl Daemon {
    pub fn run() {
        let config = Arc::new(Mutex::new(read_config()));
        Ipc::listen(config.clone());

        let daemon = Self {
            config,
            tracker: NiriTracker::new(),
        };

        daemon.events_listen();
    }

    fn events_listen(&self) {
        let mut event_reader = get_event_reader();
        loop {
            let event = event_reader();
            self.handle_event(&event);
        }
    }

    fn handle_event(&self, event: &Event) {
        match event {
            Event::WindowsChanged { windows } => self.tracker.update_windows(windows),
            Event::WindowClosed { id } => {
                self.tracker.unregister_window(id);
            }
            Event::WindowOpenedOrChanged { window } => {
                self.handle_window_opened_or_changed(window);
            }
            _ => (),
        }
    }

    fn handle_window_opened_or_changed(&self, window: &Window) {
        if self.tracker.register_window(window) {
            self.attain_config().evaluate_window(window);
        }
    }

    fn attain_config(&self) -> MutexGuard<Config> {
        self.config.lock().expect("Failed to attain config lock")
    }
}
