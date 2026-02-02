use egui::Ui;
use egui_plot::PlotPoint;

use crate::ui::App;

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

        let graphs: Vec<Vec<PlotPoint>> = vec![g_gd15, g_csm, g_dpm, g_kp];

        ui.vertical_centered_justified(|ui| {
            //username label
            let max_width = ui.available_width();
            ui.set_max_width(200.0);
            ui.label(format!(
                "Current user:\n{} ({:?})",
                self.username, self.region
            ));

            //graph grid
            ui.set_max_width(max_width);
            self.draw_stat_graph_strip(ui, &graphs, self.graph_dimensions);
        });
    }
}
