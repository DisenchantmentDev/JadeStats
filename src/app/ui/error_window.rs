use crate::ui::App;
use egui::{Align2, Color32, Frame};
use egui::{Context, RichText, Vec2, Window};

impl App {
    pub fn display_error_window(&mut self, ctx: &Context, details: &String) {
        let mut frame = Frame::window(&ctx.style());
        frame.fill = Color32::from_rgba_unmultiplied(100, 0, 0, 75);
        frame.shadow = egui::Shadow::NONE;
        Window::new("Error Display")
            .interactable(false)
            .title_bar(false)
            .anchor(Align2::RIGHT_BOTTOM, Vec2::new(-10.0, -10.0))
            .fixed_size(Vec2::new(300.0, 150.0))
            .frame(frame)
            .fade_in(true)
            .show(ctx, |ui| {
                ui.label(RichText::new(format!("There was an error: {details}")).small());
            });
    }
}
