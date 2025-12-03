use std::sync::{Mutex, MutexGuard};

use niri_ipc::Window;

pub struct NiriTracker {
    window_ids: Mutex<Vec<u64>>,
}

impl NiriTracker {
    pub fn new() -> Self {
        Self {
            window_ids: Mutex::new(Vec::new()),
        }
    }

    pub fn update_windows(&self, windows: &Vec<Window>) {
        let mut window_ids = self.attain_window_ids();
        window_ids.clear();
        for window in windows {
            window_ids.push(window.id);
        }
    }

    pub fn register_window(&self, window: &Window) -> bool {
        let mut window_ids = self.attain_window_ids();
        if !window_ids.contains(&window.id) {
            window_ids.push(window.id);
            true
        } else {
            false
        }
    }

    pub fn unregister_window(&self, id: &u64) {
        self.attain_window_ids().retain(|x| x != id);
    }

    fn attain_window_ids(&self) -> MutexGuard<'_, Vec<u64>> {
        self.window_ids
            .lock()
            .expect("Failed to attain known_window_ids lock")
    }
}
