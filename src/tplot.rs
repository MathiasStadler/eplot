use egui::{Color32, Frame, Response, Sense, Stroke, Vec2};
use egui_plot::{Line, Plot, PlotPoints};

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
}

impl Default for PlotInstance {
    fn default() -> Self {
        Self {
            title: "Plot 1".to_string(),
            height: 300.0,
            active_signals: Vec::new(),
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

    fn show_plot(&mut self, ui: &mut egui::Ui, index: usize) -> bool {
        let mut should_remove = false;
        let plot = &mut self.plots[index];

        // Title bar frame
        let title_height = 24.0;
        Frame::none()
            .stroke(Stroke::new(
                1.0,
                ui.visuals().widgets.noninteractive.bg_fill,
            ))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.set_min_height(title_height);
                    ui.label(&plot.title);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("✕").clicked() {
                            should_remove = true;
                        }
                    });
                });
            });

        // Check if something is being dragged
        let is_being_dragged = self.dragged_signal.is_some();

        // Create a frame around the plot that will be highlighted when dragging
        let frame_stroke = if is_being_dragged {
            Stroke::new(2.0, Color32::YELLOW)
        } else {
            Stroke::NONE
        };

        Frame::none().stroke(frame_stroke).show(ui, |ui| {
            // Plot content with drop target
            let plot_response = Plot::new(format!("plot_{}", index))
                .height(plot.height)
                .width(ui.available_width())
                .allow_zoom(true)
                .allow_drag(true)
                .show_axes(true)
                .show_grid(true)
                .legend(egui_plot::Legend::default())
                .show(ui, |plot_ui| {
                    for active_signal in &plot.active_signals {
                        let signal = &self.signals[active_signal.signal_index];
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
            let is_hovered = rect.contains(ui.input(|i| i.pointer.hover_pos().unwrap_or_default()));

            // Handle drops
            if is_being_dragged && is_hovered {
                // Show a stronger highlight when hovering during drag
                ui.painter()
                    .rect_stroke(rect, 0.0, Stroke::new(3.0, Color32::GREEN));

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

        // Resize handle
        let resize_id = ui.id().with(format!("resize_handle_{}", index));
        let resize_rect = egui::Rect::from_min_size(
            ui.min_rect().left_bottom() + Vec2::new(0.0, 2.0),
            Vec2::new(ui.available_width(), 5.0),
        );
        let resize_response = ui.interact(resize_rect, resize_id, egui::Sense::drag());

        if resize_response.dragged() {
            plot.height = (plot.height + resize_response.drag_delta().y)
                .max(100.0)
                .min(ui.available_height());
        }

        // Draw resize handle
        if resize_response.hovered() {
            ui.painter()
                .rect_filled(resize_rect, 0.0, ui.style().visuals.widgets.active.bg_fill);
        } else {
            ui.painter().rect_filled(
                resize_rect,
                0.0,
                ui.style().visuals.widgets.inactive.bg_fill,
            );
        }

        ui.add_space(8.0); // Space between plots
        should_remove
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
                    // let new_title = format!("Plot {}", self.plots.len() + 1);
                    self.plots.push(PlotInstance::default());
                }
            });

        // Main area with plots
        egui::CentralPanel::default()
            .show_inside(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    // Show existing plots
                    let mut i = 0;
                    while i < self.plots.len() {
                        if self.show_plot(ui, i) {
                            self.plots.remove(i);
                        } else {
                            i += 1;
                        }
                    }
                });

                ui.allocate_response(ui.available_size(), egui::Sense::hover())
            })
            .response
    }
}
