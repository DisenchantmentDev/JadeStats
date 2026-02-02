use crate::ui::{App, Regions, State};
use analyzer_core::player::Player;
use egui::Context;
use egui::TopBottomPanel;
use egui::Ui;

impl App {
    pub fn home_central_panel(&mut self, ctx: &Context, ui: &mut Ui) {
        ui.vertical_centered_justified(|ui| {
            ui.add_space(ui.available_height() / 2.5 - 80.0);
            ui.set_max_width(200.0);
            if let Some(e) = &self.err {
                let details = e.details.clone();
                self.display_error_window(ctx, &details);
                //ui.label(
                //    RichText::new(format!(
                //        "There was an error\nPlease try again\n{}",
                //        &e.details
                //    ))
                //    .color(Color32::RED),
                //);
            }

            ui.label("Enter Username");
            ui.horizontal(|ui| {
                //text enter box
                let input_box = ui.text_edit_singleline(&mut self.username);
                //region selection box
                egui::ComboBox::from_id_salt("Region_Box")
                    .width(50.0)
                    .selected_text(format!("{:?}", self.region))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.region, Regions::NA, "NA");
                        ui.selectable_value(&mut self.region, Regions::EUW, "EUW");
                        ui.selectable_value(&mut self.region, Regions::EUNE, "EUNE");
                        ui.selectable_value(&mut self.region, Regions::KR, "KR");
                        ui.selectable_value(&mut self.region, Regions::CN, "CN");
                    });

                if input_box.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.err = None;
                    self.loading_started = false;
                    self.state = State::Loading;
                }
            });
            if ui.button("Go!").clicked() {
                self.err = None;
                self.loading_started = false;
                self.state = State::Loading;
            }
        });
    }

    pub fn top_bar(&mut self, ctx: &Context) {
        TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::containers::menu::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("To Home").clicked() {
                        self.state = State::Home;
                        self.loaded_player = Player::default();
                        self.loading_started = false;
                    }
                    if self.state == State::Stats && ui.button("Reload player").clicked() {
                        match self.loaded_player.load_new_games() {
                            Ok(_) => {}
                            Err(e) => {
                                self.state = State::Home;
                                self.err = Some(e.into());
                            }
                        }
                    }
                    if ui.button("Exit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                })
            })
        });
    }
}
