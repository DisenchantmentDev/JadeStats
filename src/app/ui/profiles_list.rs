use crate::ui::{App, AppError, Regions, State};
use egui::{Context, ScrollArea, SidePanel};

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
                            let p: Vec<&str> = profile.split('#').collect();
                            if p.len() < 3 {
                                return;
                            }
                            self.username = format!("{}#{}", p[0], p[1]);
                            self.region = match p[2] {
                                "NA" => Regions::NA,
                                "EUW" => Regions::EUW,
                                "EUNE" => Regions::EUNE,
                                "KR" => Regions::KR,
                                "CN" => Regions::CN,
                                &_ => Regions::NONE,
                            };
                            self.err = None;
                            self.loading_started = false;
                            self.state = State::Loading;
                        }
                    }
                })
            });
    }
}
