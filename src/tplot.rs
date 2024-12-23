use egui::{Color32, Frame, Response, Sense, Stroke};
use egui_plot::{Line, Plot, PlotPoints};
use std::sync::atomic::{AtomicUsize, Ordering};

static PLOT_COUNTER: AtomicUsize = AtomicUsize::new(1);

fn get_next_plot_number() -> usize {
    PLOT_COUNTER.fetch_add(1, Ordering::Relaxed)
}

#[derive(Clone, Debug, PartialEq)]
struct Signal {
    name: String,
    color: Color32,
    data_fn: fn(f64) -> f64,
}

#[derive(Clone, Debug, PartialEq)]
struct ActiveSignal {
    signal_index: usize,
    color: Color32,
}

#[derive(Clone, Debug, PartialEq)]
struct PlotInstance {
    title: String,
    height: f32,
    active_signals: Vec<ActiveSignal>,
    window_pos: Option<egui::Pos2>,
    window_size: Option<egui::Vec2>,
}

impl Default for PlotInstance {
    fn default() -> Self {
        Self {
            title: format!("Plot {}", get_next_plot_number()),
            height: 300.0,
            active_signals: Vec::new(),
            window_pos: None,
            window_size: None,
        }
    }
}

#[derive(Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub(crate) struct Tplot {
    #[serde(skip)]
    signals: Vec<Signal>,
    #[serde(skip)]
    plots: Vec<PlotInstance>,
    #[serde(skip)]
    dragged_signal: Option<usize>,
}

impl Default for Signal {
    fn default() -> Self {
        Self {
            name: "Signal".to_string(),
            color: Color32::RED,
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
                    data_fn: |x| if x < 0.0 { -1.0 } else { 1.0 },
                },
                Signal {
                    name: "Sine Wave".to_string(),
                    color: Color32::BLUE,
                    data_fn: |x| x.sin(),
                },
            ];
        }
        if self.plots.is_empty() {
            self.plots.push(PlotInstance::default());
        }
    }

    fn show_signal(&mut self, ui: &mut egui::Ui, idx: usize) -> Response {
        let signal = &mut self.signals[idx];

        let mut response = ui
            .horizontal(|ui| {
                ui.add_space(4.0);
                ui.label("⣿"); // Drag handle icon
                ui.label(&signal.name);
                let mut color = signal.color;
                if ui.color_edit_button_srgba(&mut color).changed() {
                    signal.color = color;
                }
            })
            .response;

        // Make the entire row draggable
        response = response.interact(Sense::drag());

        // Handle dragging
        if response.dragged() {
            self.dragged_signal = Some(idx);
            ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);
        } else if response.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);
        }

        // Draw a frame around the signal when hovered or dragged
        if response.hovered() || response.dragged() {
            ui.painter()
                .rect_stroke(response.rect, 5.0, Stroke::new(1.0, signal.color));
        }

        response
    }

    pub(crate) fn ui(&mut self, ui: &mut egui::Ui) -> Response {
        self.ensure_signals_initialized();

        egui::SidePanel::left("signal_selector")
            .min_width(150.0)
            .show_inside(ui, |ui| {
                ui.heading("Signals");
                ui.separator();

                for idx in 0..self.signals.len() {
                    self.show_signal(ui, idx);
                    ui.add_space(4.0);
                }

                ui.separator();
                if ui.button("Add Plot").clicked() {
                    self.plots.push(PlotInstance::default());
                }
            });

        // Show plots as windows in the central area
        egui::CentralPanel::default()
            .show_inside(ui, |ui| {
                let mut plots_to_remove = Vec::new();

                // Create a confined area for the windows
                egui::Area::new(egui::Id::new("plot_windows_area"))
                    .constrain(true) // This ensures windows stay within the area
                    .show(ui.ctx(), |ui| {
                        for (idx, plot) in self.plots.iter_mut().enumerate() {
                            let mut open = true;
                            let window = egui::Window::new(&plot.title)
                                .open(&mut open)
                                .default_size([400.0, plot.height])
                                .resizable(true)
                                .constrain(true); // This ensures the window stays within its parent area

                            let window = if let Some(pos) = plot.window_pos {
                                window.current_pos(pos)
                            } else {
                                window
                            };

                            let window = if let Some(size) = plot.window_size {
                                window.default_size(size)
                            } else {
                                window
                            };

                            window.show(ui.ctx(), |ui| {
                                // Store window size
                                plot.window_size = Some(ui.available_size());

                                // Check if something is being dragged
                                let is_being_dragged = self.dragged_signal.is_some();

                                // Create a frame that will be highlighted when dragging
                                let frame_stroke = if is_being_dragged {
                                    Stroke::new(2.0, Color32::YELLOW)
                                } else {
                                    Stroke::NONE
                                };

                                Frame::none().stroke(frame_stroke).show(ui, |ui| {
                                    // Get available size for the plot
                                    let available_size = ui.available_size();
                                    plot.height = available_size.y;

                                    let plot_response = Plot::new(format!("plot_{}", idx))
                                        .height(available_size.y)
                                        .width(available_size.x)
                                        .allow_zoom(true)
                                        .allow_drag(true)
                                        .show_axes(true)
                                        .show_grid(true)
                                        .legend(egui_plot::Legend::default())
                                        .show(ui, |plot_ui| {
                                            for active_signal in &plot.active_signals {
                                                let signal =
                                                    &self.signals[active_signal.signal_index];
                                                plot_ui.line(
                                                    Line::new(PlotPoints::from_explicit_callback(
                                                        signal.data_fn,
                                                        -5.0..5.0,
                                                        200,
                                                    ))
                                                    .color(active_signal.color)
                                                    .name(&signal.name),
                                                );
                                            }
                                        });

                                    // Make the plot area a drop target and check for hover
                                    let rect = plot_response.response.rect;
                                    let is_hovered = rect.contains(
                                        ui.input(|i| i.pointer.hover_pos().unwrap_or_default()),
                                    );

                                    // Handle drops
                                    if is_being_dragged && is_hovered {
                                        // Show a stronger highlight when hovering during drag
                                        ui.painter().rect_stroke(
                                            rect,
                                            0.0,
                                            Stroke::new(3.0, Color32::GREEN),
                                        );

                                        // Check for drops
                                        if ui.input(|i| i.pointer.any_released()) {
                                            if let Some(signal_idx) = self.dragged_signal.take() {
                                                plot.active_signals.push(ActiveSignal {
                                                    signal_index: signal_idx,
                                                    color: self.signals[signal_idx].color,
                                                });
                                            }
                                        }
                                    }
                                });
                            });

                            if !open {
                                plots_to_remove.push(idx);
                            }
                        }
                    });

                // Remove closed windows
                for idx in plots_to_remove.into_iter().rev() {
                    self.plots.remove(idx);
                }

                ui.allocate_response(ui.available_size(), egui::Sense::hover())
            })
            .response
    }
}
