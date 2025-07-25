use crate::ui::audio_pages::config_pages::ConfigPage;
use crate::ui::states::audio_state::BeacnAudioState;
use crate::ui::widgets::{get_slider, toggle_button};
use beacn_lib::audio::messages::Message;
use beacn_lib::audio::messages::suppressor::SuppressorStyle::{Adaptive, Snapshot};
use beacn_lib::audio::messages::suppressor::{Suppressor, SuppressorSensitivity};
use beacn_lib::types::Percent;
use egui::Button;
use egui::Ui;

pub struct NoiseSuppressionPage;

impl ConfigPage for NoiseSuppressionPage {
    fn title(&self) -> &'static str {
        "Noise Suppression"
    }

    fn ui(&mut self, ui: &mut Ui, state: &mut BeacnAudioState) {
        let spacing = 5.0;

        let mut ns = state.suppressor;
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                if ui.checkbox(&mut ns.enabled, "Enabled").changed() {
                    let message = Message::Suppressor(Suppressor::Enabled(ns.enabled));
                    state
                        .handle_message(message)
                        .expect("Failed to Send Message");
                }

                ui.add_space(spacing);

                ui.horizontal(|ui| {
                    let size = [105., 20.];
                    let a = toggle_button(ui, ns.style == Adaptive, "Adaptive");
                    let s = toggle_button(ui, ns.style == Snapshot, "Snapshot");
                    if ui.add_sized(size, a).clicked() {
                        let message = Message::Suppressor(Suppressor::Style(Adaptive));
                        state
                            .handle_message(message)
                            .expect("Failed to Send Message");
                        ns.style = Adaptive;
                    }
                    if ui.add_sized(size, s).clicked() {
                        let message = Message::Suppressor(Suppressor::Style(Snapshot));
                        state
                            .handle_message(message)
                            .expect("Failed to Send Message");
                        ns.style = Snapshot;
                    }
                });

                ui.add_space(spacing);

                let s = get_slider(ui, "Amount", "%", &mut ns.amount, 0..=100);
                if s.changed() {
                    let value = Percent(ns.amount as f32);
                    let message = Message::Suppressor(Suppressor::Amount(value));
                    state
                        .handle_message(message)
                        .expect("Failed to Send Message");
                }

                ui.add_space(spacing);

                if ns.style == Adaptive {
                    let s = get_slider(ui, "Sensitivity", "%", &mut ns.sense, 0..=100);
                    if s.changed() {
                        let value = -120.0 + (60.0 * (ns.sense as f32 / 100.0));
                        let value = SuppressorSensitivity(value);
                        let message = Message::Suppressor(Suppressor::Sensitivity(value));
                        state
                            .handle_message(message)
                            .expect("Failed to Send Message");
                    }
                } else if ns.style == Snapshot {
                    ui.add_space(15.);

                    ui.disable();
                    // This doesn't work, but we're going to need to handle it differently anyway
                    ui.add_sized(
                        [220.0, 0.],
                        Button::selectable(true, "Snapshot Not Supported"),
                    );
                }
            });
        });
    }
}
