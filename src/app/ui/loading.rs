use egui::Ui;
use std::sync::Arc;

use crate::app::app_error::AppError;
use crate::ui::App;
use crate::ui::LoadingState;
use crate::ui::PlayerLoadCtx;
use crate::ui::State;
use egui_alignments::{center_horizontal_wrapped, center_vertical};

#[allow(clippy::allow_attributes, clippy::missing_errors_doc)]
impl App {
    pub fn start_loading(&mut self) -> Result<(), AppError> {
        if self.state == State::Loading && !self.loading_started {
            self.loading_started = true;

            let mut ctx = PlayerLoadCtx {
                username: self.username.clone(),
                region: self.region.clone(),
                root_dir: self.root_dir.clone(),
            };

            let loading_player = Arc::clone(&self.player);

            std::thread::spawn(move || {
                let result = ctx.load_player();

                let mut guard = loading_player.lock().expect("Failed mutex operation");
                *guard = match result {
                    Ok(player) => LoadingState::Loaded(player),
                    Err(e) => LoadingState::Error(e),
                };
            });
        }
        Ok(())
    }

    pub fn display_loading(&mut self, ui: &mut Ui) {
        let guard = self.player.lock().expect("Failed mutex guard check");
        match &*guard {
            LoadingState::Dormant | LoadingState::Loading => {
                center_vertical(ui, |ui| {
                    center_horizontal_wrapped(ui, |ui| {
                        ui.heading("Loading...");
                        ui.spinner();
                    });
                });
                //ui.vertical_centered_justified(|ui| {
                //    ui.heading("Loading...");
                //    ui.spinner();
                //});
            }
            LoadingState::Loaded(player) => {
                self.loaded_player = player.clone();
                self.has_loaded = true;
                self.state = State::Stats;
            }
            LoadingState::Error(app_error) => {
                self.err = Some(app_error.clone());
                self.state = State::Home;
            }
        }
    }
}
