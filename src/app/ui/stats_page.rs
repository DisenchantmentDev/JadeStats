use egui::Ui;
use egui_extras::{Size, StripBuilder};
use egui_plot::{Line, Plot, PlotPoint, PlotPoints};

use crate::ui::App;
use crate::ui::GraphType;

impl App {
    pub fn draw_stats(&mut self, ui: &mut Ui) {
        /* Generate Vec<PlotPoint> for each graph
         * Generate a graph for each one (edit how drawing the strip works)
         * draw that here */

        let g_gd15: Vec<PlotPoint> = self
            .loaded_player
            .gd15_points()
            .into_iter()
            .enumerate()
            .map(|(i, v)| PlotPoint::new(i as f64, v as f64))
            .collect();

        let g_csm: Vec<PlotPoint> = self
            .loaded_player
            .csm_points()
            .into_iter()
            .enumerate()
            .map(|(i, v)| PlotPoint::new(i as f64, v as f64))
            .collect();

        let g_dpm: Vec<PlotPoint> = self
            .loaded_player
            .dpm_points()
            .into_iter()
            .enumerate()
            .map(|(i, v)| PlotPoint::new(i as f64, v as f64))
            .collect();

        let g_kp: Vec<PlotPoint> = self
            .loaded_player
            .kp_points()
            .into_iter()
            .enumerate()
            .map(|(i, v)| PlotPoint::new(i as f64, v as f64))
            .collect();

        let sin: Vec<PlotPoint> = (0..1000)
            .map(|i| {
                let x = i as f64 * 0.01;
                PlotPoint::new(x, x.sin())
            })
            .collect();

        let cosin: Vec<PlotPoint> = (0..1000)
            .map(|i| {
                let x = i as f64 * 0.01;
                PlotPoint::new(x, x.cos())
            })
            .collect();

        let graphs: Vec<Vec<PlotPoint>> =
            vec![sin.clone(), cosin.clone(), cosin.clone(), sin.clone()];

        ui.vertical_centered_justified(|ui| {
            //username label
            let max_width = ui.available_width();
            ui.set_max_width(200.0);
            ui.label(format!("Current user: {}#{:?}", self.username, self.region));

            //graph grid
            ui.set_max_width(max_width);
            self.draw_stat_graph_strip(ui, graphs, self.graph_dimensions);
        });
    }
}
