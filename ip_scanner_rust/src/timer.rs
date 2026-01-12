use std::time::Instant;

pub struct Timer {
    start_time: Option<Instant>,
    end_time: Option<Instant>,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start_time: None,
            end_time: None,
        }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn stop(&mut self) {
        self.end_time = Some(Instant::now());
    }

    pub fn get_elapsed_time(&self) -> String {
        if let (Some(start), Some(end)) = (self.start_time, self.end_time) {
            let elapsed = end.duration_since(start);
            let total_seconds = elapsed.as_secs();
            
            let hours = total_seconds / 3600;
            let minutes = (total_seconds % 3600) / 60;
            let seconds = total_seconds % 60;
            
            format!("{} hours, {} minutes, {} seconds", hours, minutes, seconds)
        } else {
            "Timer not started or stopped properly".to_string()
        }
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}
