use egui::{Color32, Ui};
use egui_extras::{Size, StripBuilder};
use egui_plot::{Legend, Line, LineStyle, MarkerShape, Plot, PlotPoint, PlotPoints, Points};
use std::vec::Vec;

use crate::ui::App;
use crate::ui::GraphType;

#[allow(clippy::indexing_slicing, clippy::allow_attributes)]
impl App {
    pub fn draw_stat_graph_strip(
        &mut self,
        ui: &mut Ui,
        plots: &[Vec<PlotPoint>], //needs to be Vec<Vec<PlotPoint>>, or could be stored in self
        graph_dimensions: (usize, usize), //could have enum that describes graph type, and 2d array
                                  //that describes location of each graph in column/row
    ) {
        struct G {
            line: Vec<PlotPoint>,
            _type: GraphType,
        }
        /* Closure for creating plot; for scope reasons it needs to be a closure */
        let create_plot = |ui: &mut Ui, graph: GraphType, plots: Vec<PlotPoint>| {
            let plot_name = format!("{graph}");
            let curve_name = format!("{graph}");
            let plot_legend = Legend::default()
                .position(egui_plot::Corner::LeftTop)
                .title(&plot_name);
            Plot::new(plot_name)
                .allow_axis_zoom_drag(false)
                .allow_boxed_zoom(false)
                .allow_zoom(false)
                .allow_drag(false)
                .allow_scroll(false)
                .cursor_color(Color32::TRANSPARENT)
                .legend(plot_legend)
                .show(ui, |plot_ui| {
                    plot_ui.line(
                        Line::new(&curve_name, PlotPoints::Borrowed(&plots))
                            .name(&curve_name)
                            .style(LineStyle::Solid),
                    );
                    let p: PlotPoints<'_> = plots.clone().iter().map(|i| [i.x, i.y]).collect();
                    let markers = Points::new(&curve_name, p)
                        .shape(MarkerShape::Circle)
                        .radius(2.0);
                    plot_ui.points(markers);
                });
        };

        /* Same deal, but with drawing the row of graphs */
        let make_row = |ui: &mut Ui, graphs: Vec<G>| {
            StripBuilder::new(ui)
                .sizes(Size::relative((graphs.len() as f32).recip()), graphs.len())
                .horizontal(|mut strip| {
                    for g in graphs {
                        strip.cell(|ui| {
                            create_plot(ui, g._type, g.line);
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
                    make_row(
                        ui,
                        vec![
                            G {
                                line: plots[0].clone(),
                                _type: GraphType::GD15,
                            },
                            G {
                                line: plots[1].clone(),
                                _type: GraphType::CSM,
                            },
                        ],
                    );
                });
                strip.cell(|ui| {
                    make_row(
                        ui,
                        vec![
                            G {
                                line: plots[2].clone(),
                                _type: GraphType::DPM,
                            },
                            G {
                                line: plots[3].clone(),
                                _type: GraphType::KP,
                            },
                        ],
                    );
                });
            });
    }
}
