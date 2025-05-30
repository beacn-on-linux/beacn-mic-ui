pub(crate) mod about;

use beacn_lib::audio::BeacnAudioDevice;
use beacn_lib::controller::BeacnControlDevice;
use egui::Ui;
use crate::states::controller_state::ControlState;

pub trait ControllerPage {
    fn icon(&self) -> &'static str;
    fn show_on_error(&self) -> bool;
    fn ui(&mut self, ui: &mut Ui, mic: &Box<dyn BeacnControlDevice>, state: &mut ControlState);
}
