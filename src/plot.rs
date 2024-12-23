use crate::demos::charts::ChartsDemo;
use crate::demos::custom_axes::CustomAxesDemo;
use crate::demos::interaction::InteractionDemo;
use crate::demos::items::ItemsDemo;
use crate::demos::legends::LegendDemo;
use crate::demos::lines::LineDemo;
use crate::demos::linked_axes::LinkedAxesDemo;
use crate::demos::markers::MarkerDemo;
use crate::tplot::Tplot;

// ----------------------------------------------------------------------------

#[derive(PartialEq, Eq, serde::Deserialize, serde::Serialize)]
enum Panel {
    Lines,
    Markers,
    Legend,
    Charts,
    Items,
    Interaction,
    CustomAxes,
    LinkedAxes,
    Tplot,
}

impl Default for Panel {
    fn default() -> Self {
        Self::Tplot
    }
}

// ----------------------------------------------------------------------------

#[derive(Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PlotDemo {
    line_demo: LineDemo,
    marker_demo: MarkerDemo,
    legend_demo: LegendDemo,
    charts_demo: ChartsDemo,
    items_demo: ItemsDemo,
    interaction_demo: InteractionDemo,
    custom_axes_demo: CustomAxesDemo,
    linked_axes_demo: LinkedAxesDemo,
    tplot: Tplot,
    open_panel: Panel,
}

impl PlotDemo {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            egui::reset_button(ui, self, "Reset");
            ui.collapsing("Instructions", |ui| {
                ui.label("Pan by dragging, or scroll (+ shift = horizontal).");
                ui.label("Box zooming: Right click to zoom in and zoom out using a selection.");
                if cfg!(target_arch = "wasm32") {
                    ui.label("Zoom with ctrl / ⌘ + pointer wheel, or with pinch gesture.");
                } else if cfg!(target_os = "macos") {
                    ui.label("Zoom with ctrl / ⌘ + scroll.");
                } else {
                    ui.label("Zoom with ctrl + scroll.");
                }
                ui.label("Reset view with double-click.");
            });
            ui.add(crate::egui_github_link_file!());
        });
        ui.separator();
        ui.horizontal_wrapped(|ui| {
            ui.selectable_value(&mut self.open_panel, Panel::Lines, "Lines");
            ui.selectable_value(&mut self.open_panel, Panel::Markers, "Markers");
            ui.selectable_value(&mut self.open_panel, Panel::Legend, "Legend");
            ui.selectable_value(&mut self.open_panel, Panel::Charts, "Charts");
            ui.selectable_value(&mut self.open_panel, Panel::Items, "Items");
            ui.selectable_value(&mut self.open_panel, Panel::Interaction, "Interaction");
            ui.selectable_value(&mut self.open_panel, Panel::CustomAxes, "Custom Axes");
            ui.selectable_value(&mut self.open_panel, Panel::LinkedAxes, "Linked Axes");
            ui.selectable_value(&mut self.open_panel, Panel::Tplot, "Tplot");
        });
        ui.separator();

        match self.open_panel {
            Panel::Lines => {
                self.line_demo.ui(ui);
            }
            Panel::Markers => {
                self.marker_demo.ui(ui);
            }
            Panel::Legend => {
                self.legend_demo.ui(ui);
            }
            Panel::Charts => {
                self.charts_demo.ui(ui);
            }
            Panel::Items => {
                self.items_demo.ui(ui);
            }
            Panel::Interaction => {
                self.interaction_demo.ui(ui);
            }
            Panel::CustomAxes => {
                self.custom_axes_demo.ui(ui);
            }
            Panel::LinkedAxes => {
                self.linked_axes_demo.ui(ui);
            }
            Panel::Tplot => {
                self.tplot.ui(ui);
            }
        }
    }
}
