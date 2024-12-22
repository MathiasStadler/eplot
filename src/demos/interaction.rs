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

#[derive(Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub(crate) struct InteractionDemo {}

impl InteractionDemo {
    #[allow(clippy::unused_self)]
    pub(crate) fn ui(&self, ui: &mut egui::Ui) -> Response {
        let id = ui.make_persistent_id("interaction_demo");

        // This demonstrates how to read info about the plot _before_ showing it:
        let plot_memory = egui_plot::PlotMemory::load(ui.ctx(), id);
        if let Some(plot_memory) = plot_memory {
            let bounds = plot_memory.bounds();
            ui.label(format!(
                "plot bounds: min: {:.02?}, max: {:.02?}",
                bounds.min(),
                bounds.max()
            ));
        }

        let plot = Plot::new("interaction_demo").id(id).height(300.0);

        let PlotResponse {
            response,
            inner: (screen_pos, pointer_coordinate, pointer_coordinate_drag_delta, bounds, hovered),
            hovered_plot_item,
            ..
        } = plot.show(ui, |plot_ui| {
            plot_ui.line(
                Line::new(PlotPoints::from_explicit_callback(
                    move |x| x.sin(),
                    ..,
                    100,
                ))
                .color(Color32::RED)
                .id(egui::Id::new("sin")),
            );
            plot_ui.line(
                Line::new(PlotPoints::from_explicit_callback(
                    move |x| x.cos(),
                    ..,
                    100,
                ))
                .color(Color32::BLUE)
                .id(egui::Id::new("cos")),
            );

            (
                plot_ui.screen_from_plot(PlotPoint::new(0.0, 0.0)),
                plot_ui.pointer_coordinate(),
                plot_ui.pointer_coordinate_drag_delta(),
                plot_ui.plot_bounds(),
                plot_ui.response().hovered(),
            )
        });

        ui.label(format!(
            "plot bounds: min: {:.02?}, max: {:.02?}",
            bounds.min(),
            bounds.max()
        ));
        ui.label(format!(
            "origin in screen coordinates: x: {:.02}, y: {:.02}",
            screen_pos.x, screen_pos.y
        ));
        ui.label(format!("plot hovered: {hovered}"));
        let coordinate_text = if let Some(coordinate) = pointer_coordinate {
            format!("x: {:.02}, y: {:.02}", coordinate.x, coordinate.y)
        } else {
            "None".to_owned()
        };
        ui.label(format!("pointer coordinate: {coordinate_text}"));
        let coordinate_text = format!(
            "x: {:.02}, y: {:.02}",
            pointer_coordinate_drag_delta.x, pointer_coordinate_drag_delta.y
        );
        ui.label(format!("pointer coordinate drag delta: {coordinate_text}"));

        let hovered_item = if hovered_plot_item == Some(egui::Id::new("sin")) {
            "red sin"
        } else if hovered_plot_item == Some(egui::Id::new("cos")) {
            "blue cos"
        } else {
            "none"
        };
        ui.label(format!("hovered plot item: {hovered_item}"));

        response
    }
}
