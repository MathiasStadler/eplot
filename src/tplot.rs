use egui::{Color32, Response};
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
                Plot::new("signals_plot")
                    .height(300.0)
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
            })
            .response
    }
}
