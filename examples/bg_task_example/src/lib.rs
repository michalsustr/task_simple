#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::DemoApp;

use rand;
use rand::Rng;
use serde::{Deserialize, Serialize};
use web_time::Instant;

// Progress event that will be sent to main thread
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct DownloadProgress {
    speed_bytes_per_sec: f64,
}

// State to track download progress
#[derive(Default)]
pub struct DownloadState {
    start_time: Option<Instant>,
    bytes_downloaded: u64,
    is_downloading: bool,
}

impl task_simple::StateTrait for DownloadState {
    type Event = DownloadProgress;

    fn progress(&mut self) -> task_simple::StateProgress<Self::Event> {
        if !self.is_downloading {
            return task_simple::StateProgress::NothingOngoing;
        }

        if let Some(start_time) = self.start_time {
            let elapsed = start_time.elapsed().as_secs_f64();
            if elapsed > 0.0 {
                let speed = self.bytes_downloaded as f64 / elapsed;
                return task_simple::StateProgress::Event(DownloadProgress {
                    speed_bytes_per_sec: speed,
                });
            }
        }

        task_simple::StateProgress::Ongoing
    }
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct DownloadFunction {}

// Download request
#[derive(Serialize, Deserialize, Clone)]
pub struct DownloadRequest {
    url: String,
}

impl task_simple::BackgroundFunction for DownloadFunction {
    type InitialState = (); // No initial state needed
    type State = DownloadState;
    type Trigger = DownloadRequest;
    type Event = DownloadProgress;

    fn initial_state<EventSender: Fn(Self::Event)>(
        self,
        _: Self::InitialState,
        _event_sender: EventSender,
    ) -> Self::State {
        DownloadState::default()
    }

    fn trigger<EventSender: Fn(Self::Event)>(
        state: &mut Self::State,
        _trigger: Self::Trigger,
        event_sender: EventSender,
    ) {
        // Start the download
        state.start_time = Some(Instant::now());
        state.bytes_downloaded = 0;
        state.is_downloading = true;

        // Simulate download with chunks
        // In real implementation, you would use reqwest or similar to actually download
        let chunk_size = 1024 * 1024; // 1MB chunks

        // Simulate downloading 10 chunks for demonstration
        for i in 0..10 {
            // Platform-agnostic delay using spin wait instead of sleep
            let delay_ms = rand::thread_rng().gen_range(50..=150);
            let target = Instant::now() + std::time::Duration::from_millis(delay_ms);
            while Instant::now() < target {
                // Busy wait
            }

            state.bytes_downloaded += chunk_size;

            // Send progress update
            let elapsed = state.start_time.unwrap().elapsed().as_secs_f64();
            if elapsed > 0.0 {
                let speed = state.bytes_downloaded as f64 / elapsed;
                event_sender(DownloadProgress {
                    // speed_bytes_per_sec: speed,
                    // For testing purposes, show the iteration number
                    speed_bytes_per_sec: (i as f64) * 1024.0 * 1024.0,
                });
            }
        }

        state.is_downloading = false;
    }

    fn event_merge(event: &mut Self::Event, other: Self::Event) {
        // Take the most recent speed measurement
        event.speed_bytes_per_sec = other.speed_bytes_per_sec;
    }
}
