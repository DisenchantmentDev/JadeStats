use analyzer_core::player;
use egui::Ui;
use std::sync::{Arc, Mutex};

use crate::app::app_error::AppError;
use crate::ui::App;
use crate::ui::LoadingState;
use crate::ui::PlayerLoadCtx;
use crate::ui::State;

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

            //std::thread::spawn(move || {
            //    {
            //        let mut guard = loading_state.lock().unwrap();
            //        *guard = super::LoadingState::Loading;
            //    }

            //    let result = self.load_player();

            //    let mut guard = loading_state.lock().unwrap();

            //    *guard = match result {
            //        Ok(player) => LoadingState::Loaded(player),
            //        Err(e) => LoadingState::Error(e),
            //    };
            //});
        }
        Ok(())
    }

    pub fn display_loading(&mut self, ui: &mut Ui) {}
}
