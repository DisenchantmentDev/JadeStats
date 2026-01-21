use std::fmt;

use analyzer_core::player::Player;
use eframe::egui::CentralPanel;
//use eframe::egui::Window;
use egui::Context;
use egui::ScrollArea;
use egui::SidePanel;
use egui::TextStyle;
use egui::TopBottomPanel;
use egui::Ui;
use egui::{Color32, FontFamily, FontId, RichText};
use egui_extras::{Size, Strip, StripBuilder};

use crate::app::app_error::AppError;
pub mod player_interface;
pub mod stats_display;
pub mod stats_page;

pub struct App {
    username: String, //player username as they would input, ie WhaleMilk#PHUD
    region: Regions,
    state: State,
    graph_dimensions: (usize, usize), //columns / rows
    has_loaded: bool,
    loaded_player: Player,
    err: Option<AppError>, // Error field that tracks if there is an error thrown by player interface
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
enum State {
    #[default]
    Home,
    Stats,
    Profile,
    Loading,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
enum GraphType {
    #[default]
    GD15,
    CSM,
    DPM,
    KP,
}

#[derive(Debug, Default, PartialEq)]
enum Regions {
    #[default]
    NA,
    EUW,
    EUNE,
    KR,
    CN,
}

impl fmt::Display for GraphType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::GD15 => write!(f, "GD@15"),
            Self::CSM => write!(f, "CS/M"),
            Self::DPM => write!(f, "D/M"),
            Self::KP => write!(f, "KP%"),
        }
    }
}
impl Default for App {
    fn default() -> Self {
        Self {
            username: String::default(),
            region: Regions::NA,
            state: State::Home,
            graph_dimensions: (2, 2),
            has_loaded: false,
            loaded_player: Player::default(),
            err: None,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        self.ui(ctx);
    }
}

impl App {
    fn ui(&mut self, ctx: &egui::Context) {
        set_style(ctx);
        top_bar(ctx);
        //CentralPanel::default().show(ctx, |vi| vi.heading("JadeStats"));
        SidePanel::left("FileList")
            .default_width(300.0)
            .min_width(100.0)
            .max_width(500.0)
            .resizable(true)
            .show(ctx, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    let available_width = ui.available_width();
                    ui.heading("Profiles");
                    ui.separator();
                })
            });

        self.draw_central_panel(ctx);

        //Window::new("TestWindow").show(ctx, |ui| {
        //    ui.label("Test Window");
        //});
    }

    fn draw_central_panel(&mut self, ctx: &egui::Context) {
        /* drawing the central Panel w/ Graphs */
        CentralPanel::default().show(ctx, |ui| match self.state {
            State::Home => {
                self.home_central_panel(ui);
            }

            State::Stats => {
                if !self.has_loaded {
                    match self.load_player() {
                        Ok(_) => {
                            self.has_loaded = true;
                            self.draw_stats(ui);
                        }
                        Err(e) => {
                            self.state = State::Home;
                            self.err = Some(e);
                        }
                    }
                } else {
                    self.draw_stats(ui);
                }
                //graph grid
                //self.draw_stat_graph_strip(ui, sin, self.graph_dimensions);
            }
            State::Profile => {
                let temp = 100;
            }
            State::Loading => {}
        });
    }

    fn home_central_panel(&mut self, ui: &mut Ui) {
        //another ui element that we can fill with red color w/ text; separate from text box ui
        ui.vertical_centered_justified(|ui| {
            ui.set_max_width(200.0);
            //TODO: if there is an error, draw a rich text above everything, maybe in a red box?
            if let Some(e) = &self.err {
                ui.label(
                    RichText::new(format!(
                        "There was an error\nplease try again\n{}",
                        e.details
                    ))
                    .color(Color32::RED),
                );
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
                    self.state = State::Stats;
                }
            });
            if ui.button("Go!").clicked() {
                self.state = State::Stats;
            }
        });
    }
}

fn set_style(ctx: &Context) {
    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading, FontId::new(30.0, FontFamily::Monospace)),
        (TextStyle::Body, FontId::new(18.0, FontFamily::Monospace)),
        (TextStyle::Button, FontId::new(14.0, FontFamily::Monospace)),
        (TextStyle::Small, FontId::new(14.0, FontFamily::Monospace)),
    ]
    .into();
    ctx.set_style(style);
}

fn top_bar(ctx: &Context) {
    TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        egui::containers::menu::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Exit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            })
        })
    });
}
