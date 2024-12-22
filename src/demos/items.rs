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
pub(crate) struct ItemsDemo {
    #[serde(skip)]
    texture: Option<egui::TextureHandle>,
}

impl ItemsDemo {
    pub(crate) fn ui(&mut self, ui: &mut egui::Ui) -> Response {
        let n = 100;
        let mut sin_values: Vec<_> = (0..=n)
            .map(|i| remap(i as f64, 0.0..=n as f64, -TAU..=TAU))
            .map(|i| [i, i.sin()])
            .collect();

        let line = Line::new(sin_values.split_off(n / 2)).fill(-1.5);
        let polygon = Polygon::new(PlotPoints::from_parametric_callback(
            |t| (4.0 * t.sin() + 2.0 * t.cos(), 4.0 * t.cos() + 2.0 * t.sin()),
            0.0..TAU,
            100,
        ));
        let points = Points::new(sin_values).stems(-1.5).radius(1.0);

        let arrows = {
            let pos_radius = 8.0;
            let tip_radius = 7.0;
            let arrow_origins = PlotPoints::from_parametric_callback(
                |t| (pos_radius * t.sin(), pos_radius * t.cos()),
                0.0..TAU,
                36,
            );
            let arrow_tips = PlotPoints::from_parametric_callback(
                |t| (tip_radius * t.sin(), tip_radius * t.cos()),
                0.0..TAU,
                36,
            );
            Arrows::new(arrow_origins, arrow_tips)
        };

        let texture: &egui::TextureHandle = self.texture.get_or_insert_with(|| {
            ui.ctx()
                .load_texture("plot_demo", egui::ColorImage::example(), Default::default())
        });
        let image = PlotImage::new(
            texture,
            PlotPoint::new(0.0, 10.0),
            5.0 * vec2(texture.aspect_ratio(), 1.0),
        );

        let plot = Plot::new("items_demo")
            .legend(Legend::default().position(Corner::RightBottom))
            .show_x(false)
            .show_y(false)
            .data_aspect(1.0);
        plot.show(ui, |plot_ui| {
            plot_ui.hline(HLine::new(9.0).name("Lines horizontal"));
            plot_ui.hline(HLine::new(-9.0).name("Lines horizontal"));
            plot_ui.vline(VLine::new(9.0).name("Lines vertical"));
            plot_ui.vline(VLine::new(-9.0).name("Lines vertical"));
            plot_ui.line(line.name("Line with fill"));
            plot_ui.polygon(polygon.name("Convex polygon"));
            plot_ui.points(points.name("Points with stems"));
            plot_ui.text(Text::new(PlotPoint::new(-3.0, -3.0), "wow").name("Text"));
            plot_ui.text(Text::new(PlotPoint::new(-2.0, 2.5), "so graph").name("Text"));
            plot_ui.text(Text::new(PlotPoint::new(3.0, 3.0), "much color").name("Text"));
            plot_ui.text(Text::new(PlotPoint::new(2.5, -2.0), "such plot").name("Text"));
            plot_ui.image(image.name("Image"));
            plot_ui.arrows(arrows.name("Arrows"));
        })
        .response
    }
}
