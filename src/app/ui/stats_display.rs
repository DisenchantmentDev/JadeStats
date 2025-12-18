use egui::Ui;
use egui_extras::{Size, StripBuilder};
use egui_plot::{Line, Plot, PlotPoint, PlotPoints};

use crate::ui::App;

impl App {
    pub fn draw_stat_graph_strip(
        &mut self,
        ui: &mut Ui,
        plots: Vec<PlotPoint>,
        graph_dimensions: (usize, usize),
    ) {
        let create_plot =
            |ui: &mut Ui, plot_name: String, curve_name: String, plots: Vec<PlotPoint>| {
                Plot::new(plot_name).show(ui, |plot_ui| {
                    plot_ui.line(
                        Line::new(&curve_name, PlotPoints::Borrowed(&plots)).name(&curve_name),
                    );
                });
            };

        StripBuilder::new(ui)
            //TODO: factor this nightmare out into smaller functions because this makes me feel
            //physical pain
            .sizes(
                Size::relative((graph_dimensions.0 as f32).recip()),
                graph_dimensions.0,
            )
            .vertical(|mut strip| {
                for r in 0..graph_dimensions.0 {
                    strip.cell(|ui| {
                        StripBuilder::new(ui)
                            .sizes(
                                egui_extras::Size::relative((graph_dimensions.1 as f32).recip()),
                                graph_dimensions.1,
                            )
                            .horizontal(|mut strip| {
                                for c in 0..graph_dimensions.1 {
                                    strip.cell(|ui| {
                                        let i = r * graph_dimensions.0 + c;
                                        create_plot(
                                            ui,
                                            format!("Plot{i}"),
                                            format!("Curve{i}"),
                                            plots.clone(),
                                        );
                                    });
                                }
                            });
                    });
                }
            });
    }
}
