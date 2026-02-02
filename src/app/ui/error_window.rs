use crate::ui::App;
use egui::{Align2, Color32, Frame};
use egui::{Context, Pos2, RichText, Ui, Vec2, Window};

impl App {
    pub fn display_error_window(&mut self, ctx: &Context, details: &String) {
        Window::new("Error Display")
            .interactable(false)
            .title_bar(false)
            .anchor(Align2::RIGHT_BOTTOM, Vec2::new(-10.0, -10.0))
            .fixed_size(Vec2::new(200.0, 100.0))
            .frame(Frame::default().fill(Color32::RED))
            .fade_in(true)
            .show(ctx, |ui| {
                ui.label(
                    RichText::new(format!("There was an error: {details}"))
                        .color(Color32::BLACK)
                        .small(),
                );
            });
    }
}
