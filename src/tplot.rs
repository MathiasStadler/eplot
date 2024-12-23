use egui::{Color32, Response, Vec2};
use egui_plot::{Line, Plot, PlotPoints};

#[derive(Clone, Debug, PartialEq)]
struct Signal {
    name: String,
    color: Color32,
    visible: bool,
    data_fn: fn(f64) -> f64, // Temporary function for demo data
}

#[derive(Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub(crate) struct Tplot {
    #[serde(skip)]
    signals: Vec<Signal>,
    plot_height: f32,
}

impl Default for Signal {
    fn default() -> Self {
        Self {
            name: "Signal".to_string(),
            color: Color32::RED,
            visible: true,
            data_fn: |x| x.sin(),
        }
    }
}

impl Tplot {
    fn ensure_signals_initialized(&mut self) {
        if self.signals.is_empty() {
            self.signals = vec![
                Signal {
                    name: "Step Function".to_string(),
                    color: Color32::RED,
                    visible: true,
                    data_fn: |x| if x < 0.0 { -1.0 } else { 1.0 },
                },
                Signal {
                    name: "Sine Wave".to_string(),
                    color: Color32::BLUE,
                    visible: true,
                    data_fn: |x| x.sin(),
                },
            ];
            self.plot_height = 300.0;
        }
    }

    pub(crate) fn ui(&mut self, ui: &mut egui::Ui) -> Response {
        self.ensure_signals_initialized();

        egui::SidePanel::left("signal_selector")
            .min_width(150.0)
            .show_inside(ui, |ui| {
                ui.heading("Signals");
                ui.separator();

                for signal in &mut self.signals {
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut signal.visible, &signal.name);
                        ui.color_edit_button_srgba(&mut signal.color);
                    });
                }
            });

        // Main plot area in central panel
        egui::CentralPanel::default()
            .show_inside(ui, |ui| {
                let plot_response = Plot::new("signals_plot")
                    .height(self.plot_height)
                    .width(ui.available_width())
                    .allow_zoom(true)
                    .allow_drag(true)
                    .show_axes(true)
                    .show_grid(true)
                    .legend(egui_plot::Legend::default())
                    .show(ui, |plot_ui| {
                        for signal in &self.signals {
                            if signal.visible {
                                plot_ui.line(
                                    Line::new(PlotPoints::from_explicit_callback(
                                        signal.data_fn,
                                        -5.0..5.0, // Set a reasonable x-range
                                        200,       // More points for smoother curves
                                    ))
                                    .color(signal.color)
                                    .name(&signal.name),
                                );
                            }
                        }
                    });

                // Add a resize handle below the plot
                let resize_id = ui.id().with("resize_handle");
                let resize_rect = egui::Rect::from_min_size(
                    plot_response.response.rect.left_bottom() + Vec2::new(0.0, 2.0),
                    Vec2::new(plot_response.response.rect.width(), 5.0),
                );
                let resize_response = ui.interact(resize_rect, resize_id, egui::Sense::drag());

                if resize_response.dragged() {
                    self.plot_height = (self.plot_height + resize_response.drag_delta().y)
                        .max(100.0)
                        .min(ui.available_height());
                }

                // Draw the resize handle
                if resize_response.hovered() {
                    ui.painter().rect_filled(
                        resize_rect,
                        0.0,
                        ui.style().visuals.widgets.active.bg_fill,
                    );
                } else {
                    ui.painter().rect_filled(
                        resize_rect,
                        0.0,
                        ui.style().visuals.widgets.inactive.bg_fill,
                    );
                }

                plot_response.response
            })
            .response
    }
}
