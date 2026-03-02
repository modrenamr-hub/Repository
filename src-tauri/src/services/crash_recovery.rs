use std::{fs, path::PathBuf};

pub struct CrashRecovery {
    marker_path: PathBuf,
}

impl CrashRecovery {
    pub fn new(marker_path: impl Into<PathBuf>) -> Self {
        Self {
            marker_path: marker_path.into(),
        }
    }

    pub fn mark_start(&self) {
        let _ = fs::write(&self.marker_path, b"running");
    }

    pub fn mark_clean_shutdown(&self) {
        let _ = fs::remove_file(&self.marker_path);
    }

    pub fn previous_run_crashed(&self) -> bool {
        self.marker_path.exists()
    }
}
