use crate::ui::{App, Regions, State};
use analyzer_core::player::Player;
use egui::Context;
use egui::TopBottomPanel;
use egui::Ui;
//use egui::{Color32, RichText};

impl App {
    pub fn home_central_panel(&mut self, ui: &mut Ui) {
        //another ui element that we can fill with red color w/ text; separate from text box ui
        ui.vertical_centered(|ui| {
            ui.add_space(ui.available_height() / 2.0 - 80.0);

            ui.label("Enter Username");

            // Allocate a centered horizontal area
            ui.allocate_ui_with_layout(
                egui::vec2(400.0, 30.0), // Adjust width as needed
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| {
                    ui.text_edit_singleline(&mut self.username);
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
                },
            );

            if ui.button("Go!").clicked() {
                self.err = None;
                self.loading_started = false;
                self.state = State::Loading;
            }
        });
        //});
        //ui.vertical_centered_justified(|ui| {
        //    ui.set_max_width(200.0);
        //    //TODO: if there is an error, draw a rich text above everything, maybe in a red box?
        //    if let Some(e) = &self.err {
        //        ui.label(
        //            RichText::new(format!(
        //                "There was an error\nPlease try again\n{}",
        //                &e.details
        //            ))
        //            .color(Color32::RED),
        //        );
        //    }

        //    ui.label("Enter Username");
        //    ui.horizontal(|ui| {
        //        //text enter box
        //        let input_box = ui.text_edit_singleline(&mut self.username);
        //        //region selection box
        //        egui::ComboBox::from_id_salt("Region_Box")
        //            .width(50.0)
        //            .selected_text(format!("{:?}", self.region))
        //            .show_ui(ui, |ui| {
        //                ui.selectable_value(&mut self.region, Regions::NA, "NA");
        //                ui.selectable_value(&mut self.region, Regions::EUW, "EUW");
        //                ui.selectable_value(&mut self.region, Regions::EUNE, "EUNE");
        //                ui.selectable_value(&mut self.region, Regions::KR, "KR");
        //                ui.selectable_value(&mut self.region, Regions::CN, "CN");
        //            });

        //        if input_box.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
        //            self.err = None;
        //            self.loading_started = false;
        //            self.state = State::Loading;
        //            //self.state = State::Stats;
        //        }
        //    });
        //    if ui.button("Go!").clicked() {
        //        self.err = None;
        //        self.loading_started = false;
        //        self.state = State::Loading;
        //        //self.state = State::Stats;
        //    }
        //});
    }

    pub fn top_bar(&mut self, ctx: &Context) {
        TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::containers::menu::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("To Home").clicked() {
                        self.state = State::Home;
                        self.loaded_player = Player::default();
                        //self.player = Arc::new(Mutex::new(LoadingState::Dormant));
                        //self.has_loaded = false;
                        self.loading_started = false;
                    }
                    if ui.button("Exit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                })
            })
        });
    }
}
