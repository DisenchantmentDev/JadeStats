#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::home::TemplateApp;
pub use app::ui;
pub use app::ui::App;

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub trait AppWindow {
    fn is_enabled(&self, _ctx: &mut egui::Context) -> bool {
        true
    }

    fn name(&self) -> &'static str;

    fn show(&mut self, _ctx: &mut egui::Context, open: &mut bool);
}
