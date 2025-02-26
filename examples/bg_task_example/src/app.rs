use web_time::Instant;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct DemoApp {
    #[serde(skip)]
    bg_task: Option<task_simple::BackgroundTask<crate::DownloadFunction>>,
    #[serde(skip)]
    current_speed: Option<f64>,
    url: String,
}

impl Default for DemoApp {
    fn default() -> Self {
        Self {
            bg_task: Default::default(),
            current_speed: None,
            url: String::new(),
        }
    }
}

impl DemoApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let previous = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Self::default()
        };

        Self {
            bg_task: Some(task_simple::BackgroundTask::new("download_worker", ())),
            current_speed: None,
            ..previous
        }
    }
}

impl eframe::App for DemoApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("File Downloader");
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Current time:");
                ui.label(format!("{:?}", Instant::now()));
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("URL:");
                ui.text_edit_singleline(&mut self.url);
            });

            // Download button and status
            if let Some(task) = &mut self.bg_task {
                let is_ongoing = task.is_ongoing();

                if ui
                    .add_enabled(!is_ongoing, egui::Button::new("Download"))
                    .clicked()
                {
                    task.trigger(crate::DownloadRequest {
                        url: self.url.clone(),
                    });
                }

                ui.separator();

                // Check for progress updates
                if let Some(progress) = task.event() {
                    self.current_speed = Some(progress.speed_bytes_per_sec);
                }

                // Show status with speed if downloading
                if is_ongoing {
                    if let Some(speed) = self.current_speed {
                        ui.label(format!(
                            "Status: Downloading... ({:.2} MB/s)",
                            speed / (1024.0 * 1024.0)
                        ));
                    } else {
                        ui.label("Status: Starting download...");
                    }
                } else {
                    ui.label("Status: Ready");
                }
            }
        });
    }
}
