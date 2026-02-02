use crate::ui::{App, AppError, Regions, State};
use analyzer_core::player::Player;
use egui::{Context, ScrollArea, SidePanel};
use std::fs;
use std::path::{Path, PathBuf};

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
                    //ui.add(egui::Label::new("text").sense(egui::Sense::click()))
                    for profile in &self.indexed_players {
                        //if ui
                        //    .add(egui::Label::new(profile).sense(egui::Sense::click()))
                        //    .clicked()
                        //{
                        if ui.add(egui::Button::new(profile).frame(false)).clicked() {
                            let pl: Vec<&str> = profile.split('#').collect();
                            let profile_path: PathBuf = self
                                .root_dir
                                .join(format!("assets/profiles/{profile}.json"));
                            match fs::read_to_string(&profile_path) {
                                Ok(p) => {
                                    self.loaded_player
                                        .load_indexed_player(p)
                                        .unwrap_or_else(|e| {
                                            self.err = Some(e.into());
                                        });
                                    self.username = format!("{}#{}", pl[0], pl[1]);
                                    self.region = match pl[2] {
                                        "NA" => Regions::NA,
                                        "EUW" => Regions::EUW,
                                        "EUNE" => Regions::EUNE,
                                        "KR" => Regions::KR,
                                        "CN" => Regions::CN,
                                        &_ => Regions::NONE,
                                    };
                                    self.state = State::Stats;
                                }
                                Err(e) => self.err = Some(e.into()),
                            }
                        }
                    }
                })
            });
    }
}
