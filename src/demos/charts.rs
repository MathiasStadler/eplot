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


#[derive(PartialEq, Eq, serde::Deserialize, serde::Serialize)]
enum Chart {
    GaussBars,
    StackedBars,
    BoxPlot,
}

impl Default for Chart {
    fn default() -> Self {
        Self::GaussBars
    }
}

#[derive(PartialEq, serde::Deserialize, serde::Serialize)]
pub(crate) struct ChartsDemo {
    chart: Chart,
    vertical: bool,
    allow_zoom: Vec2b,
    allow_drag: Vec2b,
    allow_scroll: Vec2b,
}

impl Default for ChartsDemo {
    fn default() -> Self {
        Self {
            vertical: true,
            chart: Chart::default(),
            allow_zoom: true.into(),
            allow_drag: true.into(),
            allow_scroll: true.into(),
        }
    }
}

impl ChartsDemo {
    pub(crate) fn ui(&mut self, ui: &mut egui::Ui) -> Response {
        ScrollArea::horizontal().show(ui, |ui| {
            self.options_ui(ui);
        });
        match self.chart {
            Chart::GaussBars => self.bar_gauss(ui),
            Chart::StackedBars => self.bar_stacked(ui),
            Chart::BoxPlot => self.box_plot(ui),
        }
    }

    fn options_ui(&mut self, ui: &mut egui::Ui) -> Response {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Type:");
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.chart, Chart::GaussBars, "Histogram");
                    ui.selectable_value(&mut self.chart, Chart::StackedBars, "Stacked Bar Chart");
                    ui.selectable_value(&mut self.chart, Chart::BoxPlot, "Box Plot");
                });
                ui.label("Orientation:");
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.vertical, true, "Vertical");
                    ui.selectable_value(&mut self.vertical, false, "Horizontal");
                });
            });
            ui.vertical(|ui| {
                ui.group(|ui| {
                    ui.add_enabled_ui(self.chart != Chart::StackedBars, |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Allow zoom:");
                            ui.checkbox(&mut self.allow_zoom.x, "X");
                            ui.checkbox(&mut self.allow_zoom.y, "Y");
                        });
                    });
                    ui.horizontal(|ui| {
                        ui.label("Allow drag:");
                        ui.checkbox(&mut self.allow_drag.x, "X");
                        ui.checkbox(&mut self.allow_drag.y, "Y");
                    });
                    ui.horizontal(|ui| {
                        ui.label("Allow scroll:");
                        ui.checkbox(&mut self.allow_scroll.x, "X");
                        ui.checkbox(&mut self.allow_scroll.y, "Y");
                    });
                });
            });
        })
        .response
    }

    fn bar_gauss(&self, ui: &mut egui::Ui) -> Response {
        let mut chart = BarChart::new(
            (-395..=395)
                .step_by(10)
                .map(|x| x as f64 * 0.01)
                .map(|x| {
                    (
                        x,
                        (-x * x / 2.0).exp() / (2.0 * std::f64::consts::PI).sqrt(),
                    )
                })
                // The 10 factor here is purely for a nice 1:1 aspect ratio
                .map(|(x, f)| Bar::new(x, f * 10.0).width(0.095))
                .collect(),
        )
        .color(Color32::LIGHT_BLUE)
        .name("Normal Distribution");
        if !self.vertical {
            chart = chart.horizontal();
        }

        Plot::new("Normal Distribution Demo")
            .legend(Legend::default())
            .clamp_grid(true)
            .allow_zoom(self.allow_zoom)
            .allow_drag(self.allow_drag)
            .allow_scroll(self.allow_scroll)
            .show(ui, |plot_ui| plot_ui.bar_chart(chart))
            .response
    }

    fn bar_stacked(&self, ui: &mut egui::Ui) -> Response {
        let mut chart1 = BarChart::new(vec![
            Bar::new(0.5, 1.0).name("Day 1"),
            Bar::new(1.5, 3.0).name("Day 2"),
            Bar::new(2.5, 1.0).name("Day 3"),
            Bar::new(3.5, 2.0).name("Day 4"),
            Bar::new(4.5, 4.0).name("Day 5"),
        ])
        .width(0.7)
        .name("Set 1");

        let mut chart2 = BarChart::new(vec![
            Bar::new(0.5, 1.0),
            Bar::new(1.5, 1.5),
            Bar::new(2.5, 0.1),
            Bar::new(3.5, 0.7),
            Bar::new(4.5, 0.8),
        ])
        .width(0.7)
        .name("Set 2")
        .stack_on(&[&chart1]);

        let mut chart3 = BarChart::new(vec![
            Bar::new(0.5, -0.5),
            Bar::new(1.5, 1.0),
            Bar::new(2.5, 0.5),
            Bar::new(3.5, -1.0),
            Bar::new(4.5, 0.3),
        ])
        .width(0.7)
        .name("Set 3")
        .stack_on(&[&chart1, &chart2]);

        let mut chart4 = BarChart::new(vec![
            Bar::new(0.5, 0.5),
            Bar::new(1.5, 1.0),
            Bar::new(2.5, 0.5),
            Bar::new(3.5, -0.5),
            Bar::new(4.5, -0.5),
        ])
        .width(0.7)
        .name("Set 4")
        .stack_on(&[&chart1, &chart2, &chart3]);

        if !self.vertical {
            chart1 = chart1.horizontal();
            chart2 = chart2.horizontal();
            chart3 = chart3.horizontal();
            chart4 = chart4.horizontal();
        }

        Plot::new("Stacked Bar Chart Demo")
            .legend(Legend::default())
            .data_aspect(1.0)
            .allow_drag(self.allow_drag)
            .show(ui, |plot_ui| {
                plot_ui.bar_chart(chart1);
                plot_ui.bar_chart(chart2);
                plot_ui.bar_chart(chart3);
                plot_ui.bar_chart(chart4);
            })
            .response
    }

    fn box_plot(&self, ui: &mut egui::Ui) -> Response {
        let yellow = Color32::from_rgb(248, 252, 168);
        let mut box1 = BoxPlot::new(vec![
            BoxElem::new(0.5, BoxSpread::new(1.5, 2.2, 2.5, 2.6, 3.1)).name("Day 1"),
            BoxElem::new(2.5, BoxSpread::new(0.4, 1.0, 1.1, 1.4, 2.1)).name("Day 2"),
            BoxElem::new(4.5, BoxSpread::new(1.7, 2.0, 2.2, 2.5, 2.9)).name("Day 3"),
        ])
        .name("Experiment A");

        let mut box2 = BoxPlot::new(vec![
            BoxElem::new(1.0, BoxSpread::new(0.2, 0.5, 1.0, 2.0, 2.7)).name("Day 1"),
            BoxElem::new(3.0, BoxSpread::new(1.5, 1.7, 2.1, 2.9, 3.3))
                .name("Day 2: interesting")
                .stroke(Stroke::new(1.5, yellow))
                .fill(yellow.linear_multiply(0.2)),
            BoxElem::new(5.0, BoxSpread::new(1.3, 2.0, 2.3, 2.9, 4.0)).name("Day 3"),
        ])
        .name("Experiment B");

        let mut box3 = BoxPlot::new(vec![
            BoxElem::new(1.5, BoxSpread::new(2.1, 2.2, 2.6, 2.8, 3.0)).name("Day 1"),
            BoxElem::new(3.5, BoxSpread::new(1.3, 1.5, 1.9, 2.2, 2.4)).name("Day 2"),
            BoxElem::new(5.5, BoxSpread::new(0.2, 0.4, 1.0, 1.3, 1.5)).name("Day 3"),
        ])
        .name("Experiment C");

        if !self.vertical {
            box1 = box1.horizontal();
            box2 = box2.horizontal();
            box3 = box3.horizontal();
        }

        Plot::new("Box Plot Demo")
            .legend(Legend::default())
            .allow_zoom(self.allow_zoom)
            .allow_drag(self.allow_drag)
            .show(ui, |plot_ui| {
                plot_ui.box_plot(box1);
                plot_ui.box_plot(box2);
                plot_ui.box_plot(box3);
            })
            .response
    }
}
