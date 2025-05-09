use crate::pages::MicPage;
use crate::pages::config_pages::ConfigPage;
use crate::pages::config_pages::compressor::CompressorPage;
use crate::pages::config_pages::equaliser::Equaliser;
use crate::pages::config_pages::expander::ExpanderPage;
use crate::pages::config_pages::headphones::HeadphonesPage;
use crate::pages::config_pages::mic_setup::MicSetupPage;
use crate::pages::config_pages::suppressor::NoiseSuppressionPage;
use crate::state::BeacnMicState;
use crate::widgets::draw_range;
use beacn_mic_lib::device::BeacnMic;
use beacn_mic_lib::messages::headphones::HPMicOutputGain;
use beacn_mic_lib::types::HasRange;
use egui::{Ui, vec2};
use log::debug;

pub struct Configuration {
    equaliser: Box<Equaliser>,

    selected_tab: usize,
    tab_pages: Vec<Box<dyn ConfigPage>>,
}

impl Configuration {
    pub fn new() -> Self {


        Self {
            equaliser: Box::new(Equaliser),

            selected_tab: 0,
            tab_pages: vec![
                Box::new(MicSetupPage),
                Box::new(NoiseSuppressionPage),
                Box::new(ExpanderPage),
                Box::new(CompressorPage),
                Box::new(HeadphonesPage),
            ],
        }
    }
}

impl MicPage for Configuration {
    fn icon(&self) -> &'static str {
        "mic"
    }

    fn ui(&mut self, ui: &mut Ui, mic: &BeacnMic, state: &mut BeacnMicState) {
        let eq_size = vec2(ui.available_width(), ui.available_height() - 240.);
        ui.allocate_ui_with_layout(eq_size, *ui.layout(), |ui| {
            ui.set_min_size(eq_size);
            ui.set_max_size(eq_size);
            self.equaliser.ui(ui, mic, state);
        });

        ui.separator();

        ui.vertical(|ui| {
            // 🧩 Bottom half
            let total_available = ui.available_size(); // <- how much space is left
            let fixed_panel_width = 100.0; // <- you can adjust this width
            let tab_area_width = total_available.x - fixed_panel_width;

            ui.horizontal(|ui| {
                // Left: Tab bar + active tab
                ui.allocate_ui(egui::vec2(tab_area_width, total_available.y), |ui| {
                    ui.vertical(|ui| {
                        // Tab bar
                        ui.horizontal(|ui| {
                            for (i, page) in self.tab_pages.iter().enumerate() {
                                if ui
                                    .selectable_label(self.selected_tab == i, page.title())
                                    .clicked()
                                {
                                    self.selected_tab = i;
                                }
                            }
                        });

                        ui.separator();

                        // Active tab content
                        if let Some(page) = self.tab_pages.get_mut(self.selected_tab) {
                            page.ui(ui, mic, state);
                        }
                    });
                });

                ui.separator();

                // Right: Fixed panel
                ui.allocate_ui(egui::vec2(fixed_panel_width, total_available.y), |ui| {
                    let gain = &mut state.headphones;
                    if draw_range(
                        ui,
                        &mut gain.output_gain,
                        HPMicOutputGain::range(),
                        "Output Gain",
                        "dB",
                    ) {}
                });
            });
        });
    }
}
