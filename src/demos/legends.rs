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
pub(crate) struct LegendDemo {
    config: Legend,
}

impl LegendDemo {
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

    pub(crate) fn ui(&mut self, ui: &mut egui::Ui) -> Response {
        ScrollArea::horizontal().show(ui, |ui| {
            self.settings_ui(ui);
        });

        let Self { config } = self;
        let legend_plot = Plot::new("legend_demo")
            .legend(config.clone())
            .data_aspect(1.0);
        legend_plot
            .show(ui, |plot_ui| {
                plot_ui.line(Self::line_with_slope(0.5).name("lines"));
                plot_ui.line(Self::line_with_slope(1.0).name("lines"));
                plot_ui.line(Self::line_with_slope(2.0).name("lines"));
                plot_ui.line(Self::sin().name("sin(x)"));
                plot_ui.line(Self::cos().name("cos(x)"));
            })
            .response
    }

    fn settings_ui(&mut self, ui: &mut egui::Ui) {
        let Self { config } = self;
        egui::Grid::new("settings").show(ui, |ui| {
            ui.label("Text style:");
            ui.horizontal(|ui| {
                let all_text_styles = ui.style().text_styles();
                for style in all_text_styles {
                    ui.selectable_value(&mut config.text_style, style.clone(), style.to_string());
                }
            });
            ui.end_row();

            ui.label("Position:");
            ui.horizontal(|ui| {
                Corner::all().for_each(|position| {
                    ui.selectable_value(&mut config.position, position, format!("{position:?}"));
                });
            });
            ui.end_row();

            ui.label("Opacity:");
            ui.add(
                egui::DragValue::new(&mut config.background_alpha)
                    .speed(0.02)
                    .range(0.0..=1.0),
            );
            ui.end_row();
        });
    }
}

// ----------------------------------------------------------------------------
