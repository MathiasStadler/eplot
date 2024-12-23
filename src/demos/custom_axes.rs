use std::ops::RangeInclusive;

use egui::Response;

use egui_plot::{AxisHints, GridInput, GridMark, Line, Plot, PlotPoint, PlotPoints};

#[derive(Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub(crate) struct CustomAxesDemo {}

impl CustomAxesDemo {
    const MINS_PER_DAY: f64 = 24.0 * 60.0;
    const MINS_PER_H: f64 = 60.0;

    fn logistic_fn() -> Line {
        fn days(min: f64) -> f64 {
            CustomAxesDemo::MINS_PER_DAY * min
        }

        let values = PlotPoints::from_explicit_callback(
            move |x| 1.0 / (1.0 + (-2.5 * (x / Self::MINS_PER_DAY - 2.0)).exp()),
            days(0.0)..days(5.0),
            100,
        );
        Line::new(values)
    }

    #[allow(clippy::needless_pass_by_value)]
    fn x_grid(input: GridInput) -> Vec<GridMark> {
        // Note: this always fills all possible marks. For optimization, `input.bounds`
        // could be used to decide when the low-interval grids (minutes) should be added.

        let mut marks = vec![];

        let (min, max) = input.bounds;
        let min = min.floor() as i32;
        let max = max.ceil() as i32;

        for i in min..=max {
            let step_size = if i % Self::MINS_PER_DAY as i32 == 0 {
                // 1 day
                Self::MINS_PER_DAY
            } else if i % Self::MINS_PER_H as i32 == 0 {
                // 1 hour
                Self::MINS_PER_H
            } else if i % 5 == 0 {
                // 5min
                5.0
            } else {
                // skip grids below 5min
                continue;
            };

            marks.push(GridMark {
                value: i as f64,
                step_size,
            });
        }

        marks
    }

    #[allow(clippy::unused_self)]
    pub(crate) fn ui(&self, ui: &mut egui::Ui) -> Response {
        const MINS_PER_DAY: f64 = CustomAxesDemo::MINS_PER_DAY;
        const MINS_PER_H: f64 = CustomAxesDemo::MINS_PER_H;

        fn day(x: f64) -> f64 {
            (x / MINS_PER_DAY).floor()
        }

        fn hour(x: f64) -> f64 {
            (x.rem_euclid(MINS_PER_DAY) / MINS_PER_H).floor()
        }

        fn minute(x: f64) -> f64 {
            x.rem_euclid(MINS_PER_H).floor()
        }

        fn percent(y: f64) -> f64 {
            100.0 * y
        }

        let time_formatter = |mark: GridMark, _range: &RangeInclusive<f64>| {
            let minutes = mark.value;
            if !(0.0..5.0 * MINS_PER_DAY).contains(&minutes) {
                // No labels outside value bounds
                String::new()
            } else if is_approx_integer(minutes / MINS_PER_DAY) {
                // Days
                format!("Day {}", day(minutes))
            } else {
                // Hours and minutes
                format!("{h}:{m:02}", h = hour(minutes), m = minute(minutes))
            }
        };

        let percentage_formatter = |mark: GridMark, _range: &RangeInclusive<f64>| {
            let percent = 100.0 * mark.value;
            if is_approx_zero(percent) {
                String::new() // skip zero
            } else if is_approx_integer(percent) {
                // Display only integer percentages
                format!("{percent:.0}%")
            } else {
                String::new()
            }
        };

        let label_fmt = |_s: &str, val: &PlotPoint| {
            format!(
                "Day {d}, {h}:{m:02}\n{p:.2}%",
                d = day(val.x),
                h = hour(val.x),
                m = minute(val.x),
                p = percent(val.y)
            )
        };

        ui.label("Zoom in on the X-axis to see hours and minutes");

        let x_axes = vec![
            AxisHints::new_x()
                .label("Time")
                .formatter(time_formatter)
                .placement(egui_plot::VPlacement::Top),
            AxisHints::new_x().label("Time").formatter(time_formatter),
            AxisHints::new_x().label("Value"),
        ];
        let y_axes = vec![
            AxisHints::new_y()
                .label("Percent")
                .formatter(percentage_formatter),
            AxisHints::new_y()
                .label("Absolute")
                .placement(egui_plot::HPlacement::Right),
        ];
        Plot::new("custom_axes")
            .data_aspect(2.0 * MINS_PER_DAY as f32)
            .custom_x_axes(x_axes)
            .custom_y_axes(y_axes)
            .x_grid_spacer(Self::x_grid)
            .label_formatter(label_fmt)
            .show(ui, |plot_ui| {
                plot_ui.line(Self::logistic_fn());
            })
            .response
    }
}

fn is_approx_zero(val: f64) -> bool {
    val.abs() < 1e-6
}

fn is_approx_integer(val: f64) -> bool {
    val.fract().abs() < 1e-6
}
