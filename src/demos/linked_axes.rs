use std::f64::consts::TAU;
use std::ops::RangeInclusive;

use egui::{
    remap, vec2, Color32, ComboBox, NumExt, Pos2, Response, ScrollArea, Stroke, TextWrapMode, Vec2b,
};

use egui_plot::{
    Arrows, AxisHints, Bar, BarChart, BoxElem, BoxPlot, BoxSpread, CoordinatesFormatter, Corner,
    GridInput, GridMark, HLine, Legend, Line, LineStyle, MarkerShape, Plot, PlotImage, PlotPoint,
    PlotPoints, PlotResponse, Points, Polygon, Text, VLine,
};


#[derive(PartialEq, serde::Deserialize, serde::Serialize)]
pub(crate) struct LinkedAxesDemo {
    link_axis: Vec2b,
    link_cursor: Vec2b,
}

impl Default for LinkedAxesDemo {
    fn default() -> Self {
        Self {
            link_axis: Vec2b::new(true, true),
            link_cursor: Vec2b::new(true, true),
        }
    }
}

impl LinkedAxesDemo {
    fn line_with_slope(slope: f64) -> Line {
        Line::new(PlotPoints::from_explicit_callback(
            move |x| slope * x,
            ..,
            100,
        ))
    }

    fn sin() -> Line {
        Line::new(PlotPoints::from_explicit_callback(
            move |x| x.sin(),
            ..,
            100,
        ))
    }

    fn cos() -> Line {
        Line::new(PlotPoints::from_explicit_callback(
            move |x| x.cos(),
            ..,
            100,
        ))
    }

    fn configure_plot(plot_ui: &mut egui_plot::PlotUi) {
        plot_ui.line(Self::line_with_slope(0.5));
        plot_ui.line(Self::line_with_slope(1.0));
        plot_ui.line(Self::line_with_slope(2.0));
        plot_ui.line(Self::sin());
        plot_ui.line(Self::cos());
    }

    pub(crate) fn ui(&mut self, ui: &mut egui::Ui) -> Response {
        ui.horizontal(|ui| {
            ui.label("Linked axes:");
            ui.checkbox(&mut self.link_axis.x, "X");
            ui.checkbox(&mut self.link_axis.y, "Y");
        });
        ui.horizontal(|ui| {
            ui.label("Linked cursors:");
            ui.checkbox(&mut self.link_cursor.x, "X");
            ui.checkbox(&mut self.link_cursor.y, "Y");
        });

        ScrollArea::horizontal()
            .show(ui, |ui| self.plots_ui(ui))
            .inner
    }

    pub(crate) fn plots_ui(&self, ui: &mut egui::Ui) -> Response {
        ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);
        let link_group_id = ui.id().with("linked_demo");
        ui.horizontal(|ui| {
            Plot::new("left-top")
                .data_aspect(1.0)
                .width(250.0)
                .height(250.0)
                .link_axis(link_group_id, self.link_axis)
                .link_cursor(link_group_id, self.link_cursor)
                .show(ui, Self::configure_plot);
            Plot::new("right-top")
                .data_aspect(2.0)
                .width(150.0)
                .height(250.0)
                .y_axis_label("y")
                .y_axis_position(egui_plot::HPlacement::Right)
                .link_axis(link_group_id, self.link_axis)
                .link_cursor(link_group_id, self.link_cursor)
                .show(ui, Self::configure_plot);
        });
        Plot::new("left-bottom")
            .data_aspect(0.5)
            .width(250.0)
            .height(150.0)
            .x_axis_label("x")
            .link_axis(link_group_id, self.link_axis)
            .link_cursor(link_group_id, self.link_cursor)
            .show(ui, Self::configure_plot)
            .response
    }
}
