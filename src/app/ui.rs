use eframe::egui::CentralPanel;
use egui::Context;
use egui::TextStyle;
use egui::TopBottomPanel;
use egui::{FontFamily, FontId};
use egui_plot::{Line, Plot, PlotPoints};

#[derive(Default)]
pub struct App {
    username: String, //player username as they would input, ie WhaleMilk#PHUD
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        set_style(ctx);
        top_bar(ctx);
        //CentralPanel::default().show(ctx, |vi| vi.heading("JadeStats"));
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.set_max_width(200.0);
                ui.label("Username#PID");
                ui.add(egui::TextEdit::singleline(&mut self.username).clip_text(true));

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
