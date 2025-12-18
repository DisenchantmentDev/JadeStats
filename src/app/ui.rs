use std::fmt;
use std::ptr::null;

use analyzer_core::player::Player;
use eframe::egui::CentralPanel;
use eframe::egui::Window;
use egui::Context;
use egui::Grid;
use egui::ScrollArea;
use egui::SidePanel;
use egui::TextEdit;
use egui::TextStyle;
use egui::TopBottomPanel;
use egui::Ui;
use egui::{FontFamily, FontId};
use egui_extras::{Size, Strip, StripBuilder};
use egui_plot::{Line, Plot, PlotPoint, PlotPoints};

pub mod stats_display;

pub struct App {
    username: String, //player username as they would input, ie WhaleMilk#PHUD
    state: State,
    graph_dimensions: (usize, usize), //columns / rows
    loaded_player: Player,
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
            state: State::Home,
            graph_dimensions: (2, 2),
            loaded_player: Player::default(),
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
        CentralPanel::default().show(ctx, |ui| match self.state {
            State::Home => {
                self.home_central_panel(ui);
            }
            State::Stats => {
                let sin: Vec<PlotPoint> = (0..1000)
                    .map(|i| {
                        let x = i as f64 * 0.01;
                        PlotPoint::new(x, x.sin())
                    })
                    .collect();
                ui.vertical_centered_justified(|ui| {
                    //username label
                    let max_width = ui.available_width();
                    ui.set_max_width(200.0);
                    ui.label(format!("Current username: {}", self.username));

                    //graph grid
                    ui.set_max_width(max_width);
                    self.draw_stat_graph_strip(ui, sin, self.graph_dimensions);
                    //StripBuilder::new(ui)
                    //    //TODO: factor this nightmare out into smaller functions because this makes me feel
                    //    //physical pain
                    //    .sizes(
                    //        Size::relative((self.graph_dimensions.0 as f32).recip()),
                    //        self.graph_dimensions.0,
                    //    )
                    //    .vertical(|mut strip| {
                    //        for r in 0..self.graph_dimensions.0 {
                    //            strip.cell(|ui| {
                    //                StripBuilder::new(ui)
                    //                    .sizes(
                    //                        egui_extras::Size::relative(
                    //                            (self.graph_dimensions.1 as f32).recip(),
                    //                        ),
                    //                        self.graph_dimensions.1,
                    //                    )
                    //                    .horizontal(|mut strip| {
                    //                        for c in 0..self.graph_dimensions.1 {
                    //                            strip.cell(|ui| {
                    //                                let i = r * self.graph_dimensions.0 + c;
                    //                                Plot::new(format!("Plot{i}")).show(
                    //                                    ui,
                    //                                    |plot_ui| {
                    //                                        plot_ui.line(
                    //                                            Line::new(
                    //                                                format!("curve{i}"),
                    //                                                PlotPoints::Borrowed(&sin),
                    //                                            )
                    //                                            .name(format!("curve{i}")),
                    //                                        );
                    //                                    },
                    //                                );
                    //                            });
                    //                        }
                    //                    });
                    //            });
                    //        }
                    //    });
                });
            }
            State::Profile => {}
            State::Loading => {}
        });
    }

    fn home_central_panel(&mut self, ui: &mut Ui) {
        ui.vertical_centered_justified(|ui| {
            ui.set_max_width(200.0);
            ui.label("Enter Username");
            let input_box = ui.text_edit_singleline(&mut self.username);
            if input_box.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                self.state = State::Stats;
            }
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
