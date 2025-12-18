use egui::Ui;
use egui_extras::{Size, StripBuilder};
use egui_plot::{Line, Plot, PlotPoint, PlotPoints};

use crate::ui::App;
use crate::ui::GraphType;

impl App {
    pub fn draw_stat_graph_strip(
        &mut self,
        ui: &mut Ui,
        plots: Vec<PlotPoint>, //needs to be Vec<Vec<PlotPoint>>, or could be stored in self
        graph_dimensions: (usize, usize), //could have enum that describes graph type, and 2d array
                               //that describes location of each graph in column/row
    ) {
        /* Closure for creating plot; for scope reasons it needs to be a closure */
        let create_plot = |ui: &mut Ui, graph: GraphType, plots: Vec<PlotPoint>| {
            let plot_name = format!("{graph}");
            let curve_name = format!("{graph}_Curve");
            Plot::new(plot_name).show(ui, |plot_ui| {
                plot_ui
                    .line(Line::new(&curve_name, PlotPoints::Borrowed(&plots)).name(&curve_name));
            });
        };

        /* Same deal, but with drawing the row of graphs */
        let make_row = |ui: &mut Ui, graphs: Vec<GraphType>| {
            StripBuilder::new(ui)
                .sizes(Size::relative((graphs.len() as f32).recip()), graphs.len())
                .horizontal(|mut strip| {
                    for g in graphs {
                        strip.cell(|ui| {
                            create_plot(ui, g, plots.clone());
                        });
                    }
                });
        };

        /* Built a strip of graphs. Revisit after implementing data collection */
        StripBuilder::new(ui)
            .sizes(
                Size::relative((graph_dimensions.0 as f32).recip()),
                graph_dimensions.0,
            )
            .vertical(|mut strip| {
                strip.cell(|ui| {
                    make_row(ui, vec![GraphType::GD15, GraphType::CSM]);
                });
                strip.cell(|ui| {
                    make_row(ui, vec![GraphType::DPM, GraphType::KP]);
                });
            });
    }
}
