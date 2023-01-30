#![windows_subsystem = "windows"]

use iced::widget::text_input;
use once_cell::sync::Lazy;

mod gui;
mod keymap;
mod keymap_entry;
mod types;

static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);
const ROBOTO: [u8; 182172] = *include_bytes!("../resources/RobotoMono-VariableFont_wght.ttf");

pub fn main() -> iced::Result {
  gui::main()
}
