use crate::ui::{App, AppError, PlayerLoadCtx, Regions, State};
use egui::{Context, ScrollArea, SidePanel};
use std::fs;
use std::path::PathBuf;

#[allow(clippy::indexing_slicing, clippy::allow_attributes)]
impl App {
    pub fn draw_side_panel(&mut self, ctx: &Context) {
        SidePanel::left("FileList")
            .default_width(300.0)
            .min_width(100.0)
            .max_width(500.0)
            .resizable(true)
            .show(ctx, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    let _available_width = ui.available_width();
                    ui.heading("Profiles");
                    ui.separator();
                    for profile in &self.indexed_players {
                        if ui.add(egui::Button::new(profile).frame(false)).clicked() {
                            self.err = None;
                            let pl: Vec<&str> = profile.split('#').collect();
                            let profile_path: PathBuf = self
                                .root_dir
                                .join(format!("assets/profiles/{profile}.json"));
                            if let Ok(p) = fs::read_to_string(&profile_path) {
                                self.loaded_player
                                    .load_indexed_player(p)
                                    .unwrap_or_else(|e| {
                                        self.err = Some(e.into());
                                    });
                                self.username = format!(
                                    "{}#{}",
                                    pl.first().expect("Failed to index"),
                                    pl.get(1).expect("failed to index")
                                );
                                self.region = match pl[2] {
                                    "NA" => Regions::NA,
                                    "EUW" => Regions::EUW,
                                    "EUNE" => Regions::EUNE,
                                    "KR" => Regions::KR,
                                    "CN" => Regions::CN,
                                    &_ => Regions::NONE,
                                };
                                self.state = State::Stats;
                            } else {
                                self.err =
                                    Some(AppError::new("Could not find player file. Removing"));
                                PlayerLoadCtx::remove_player_file(&self.root_dir, profile)
                                    .expect("Could not open index file");
                                self.update_index_players = true;
                            }
                        }
                    }
                })
            });
    }
}
