use serde::{Deserialize, Serialize};
use tauri::{Manager, Window};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Progress {
    pub filesize: Option<u64>,
    pub transfered: u64,
    pub transfer_rate: f64,
    pub percentage: f64,
}

impl Progress {
    pub fn emit_progress(&self, window: &Window) {
        window.emit_all("download_progress", &self.percentage).ok();
    }

    pub fn emit_finished(&self, window: &Window) {
        window.emit_all("download_finished", &self.percentage).ok();
    }
}