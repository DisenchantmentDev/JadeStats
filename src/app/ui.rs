use eframe::egui::CentralPanel;
use eframe::egui::Window;
use egui::Context;
use egui::SidePanel;
use egui::TextEdit;
use egui::TextStyle;
use egui::TopBottomPanel;
use egui::{FontFamily, FontId};
use egui_plot::{Line, Plot, PlotPoints};

pub struct App {
    username: String, //player username as they would input, ie WhaleMilk#PHUD
    state: State,     //Coul
    test_bool: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
enum State {
    #[default]
    Home,
    Stats,
    Profile,
    Loading,
}

impl Default for App {
    fn default() -> Self {
        Self {
            username: String::default(),
            state: State::Home,
            test_bool: false,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        set_style(ctx);
        top_bar(ctx);
        //CentralPanel::default().show(ctx, |vi| vi.heading("JadeStats"));
        CentralPanel::default().show(ctx, |ui| {
            //match self.state {
            //    State::Home =>
            //}
            if self.state == State::Home {
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

            if self.state == State::Stats {
                ui.vertical_centered_justified(|ui| {
                    ui.label(format!("Current username: {}", self.username));
                    ui.set_max_width(600.0);

                    let sin: PlotPoints = (0..100)
                        .map(|i| {
                            let x = i as f64 * 0.1;
                            [x, x.sin()]
                        })
                        .collect();
                    let line = Line::new("sin_wave", sin);

                    Plot::new("test_plot")
                        .view_aspect(2.0)
                        .show(ui, |plot_ui| plot_ui.line(line));
                });
            }
        });

        SidePanel::left("FileList")
            .default_width(200.0)
            .max_width(400.0)
            .min_width(200.0)
            .show(ctx, |ui| {
                ui.label("Saved Profiles");
            });

        Window::new("TestWindow").show(ctx, |ui| {
            ui.label("Test Window");
        });
    }
}

impl App {}

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
    TopBottomPanel::top("menu_bar").show(ctx, |vi| {
        egui::containers::menu::MenuBar::new().ui(vi, |vi| {
            vi.menu_button("File", |vi| {
                if vi.button("Exit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            })
        })
    });
}
