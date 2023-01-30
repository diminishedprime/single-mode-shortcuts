#![windows_subsystem = "windows"]

use clap::Parser;
use iced::alignment;
use iced::color;
use iced::event;
use iced::keyboard;
use iced::subscription;
use iced::theme;
use iced::theme::Theme;

use iced::widget::{self, column, container, row, text, text_input};
use iced::window;

use iced::Event;
use iced::Font;
use iced::{Application, Element};
use iced::{Command, Length, Settings, Subscription};
use keymap::get_keymap;
use keymap_entry::KeymapEntry;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::str;

mod keymap;
mod keymap_entry;

static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);

const ROBOTO: [u8; 182172] = *include_bytes!("../resources/RobotoMono-VariableFont_wght.ttf");

#[derive(Parser, Debug)]
#[command(author, version, about, long_about= None)]
struct Args {
  /// Initial state to the input. For example "g" will go straight to the go_to mode.
  #[arg(short, long, default_value_t = String::new())]
  input: String,
}

pub fn main() -> iced::Result {
  SingleModeShortcuts::run(Settings {
    window: window::Settings {
      size: (500, 300),
      decorations: false,
      ..window::Settings::default()
    },
    ..Settings::default()
  })
}

#[derive(Debug)]
enum SingleModeShortcuts {
  Loading,
  Loaded(State),
}

#[derive(Debug, Default, Clone)]
struct State {
  input_value: String,
  keymap: KeymapEntry,
}

impl State {
  fn new() -> Self {
    let args = Args::parse();
    State {
      input_value: args.input,
      keymap: get_keymap(),
    }
  }

  fn maps(&self) -> Element<Message> {
    let current_map = self
      .input_value
      .chars()
      .fold(Some(&self.keymap), |acc, key| match acc {
        Some(current_map) => match current_map {
          KeymapEntry::Leaf { .. } => None,
          KeymapEntry::Node { map, .. } => map.get(&*key.to_string()),
        },
        None => None,
      });
    if let Some(KeymapEntry::Node { map, .. }) = current_map {
      column(
        map
          .iter()
          .map(|(key, value)| {
            let key = if key == &" " { "<space>" } else { key };
            let key = text(format!("{key: >7}"))
              .font(Font::External {
                name: "Roboto",
                bytes: &ROBOTO,
              })
              .size(16)
              .style(color!(0xcb4b16));

            let value = text(value.get_name())
              .font(Font::External {
                name: "Roboto",
                bytes: &ROBOTO,
              })
              .size(16)
              .style(if value.is_mode() {
                color!(0x2aa198)
              } else {
                color!(0xfdf6e3)
              });

            row![key, value].spacing(8).into()
          })
          .collect(),
      )
      .into()
    } else {
      column![].into()
    }
  }
}

#[derive(Debug, Clone)]
enum Message {
  Loaded(State),
  InputChanged(String),
  TabPressed { shift: bool },
  CreateTask,
}

impl Application for SingleModeShortcuts {
  type Message = Message;
  type Theme = Theme;
  type Executor = iced::executor::Default;
  type Flags = ();

  fn new(_flags: ()) -> (SingleModeShortcuts, Command<Message>) {
    (
      SingleModeShortcuts::Loading,
      Command::perform(State::load(), Message::Loaded),
    )
  }

  fn title(&self) -> String {
    "Single Mode Shortcuts".to_string()
  }

  fn update(&mut self, message: Message) -> Command<Message> {
    match self {
      SingleModeShortcuts::Loaded(state) => {
        let command = match message {
          Message::InputChanged(value) => {
            println!("Input changed: {}", &value);

            let current_map = value
              .chars()
              .fold(Some(&state.keymap), |acc, key| match acc {
                Some(current_map) => match current_map {
                  KeymapEntry::Leaf { .. } => None,
                  KeymapEntry::Node { map, .. } => map.get(&*key.to_string()),
                },
                None => None,
              });

            // If we're on a node, run it right away.
            if let Some(KeymapEntry::Leaf(leaf)) = current_map {
              leaf.run().unwrap();
            }

            if !matches!(
              current_map,
              Some(KeymapEntry::Leaf(keymap_entry::Leaf::LaunchNoQuit(_)))
            ) {
              state.input_value = value;
            }

            Command::none()
          }
          Message::TabPressed { shift } => {
            if shift {
              widget::focus_previous()
            } else {
              widget::focus_next()
            }
          }
          Message::CreateTask => {
            state.keymap.run(&state.input_value);
            Command::none()
          }
          _ => Command::none(),
        };

        Command::batch(vec![command])
      }
      SingleModeShortcuts::Loading => {
        *self = SingleModeShortcuts::Loaded(State::new());
        text_input::focus(INPUT_ID.clone())
      }
    }
  }

  fn theme(&self) -> Self::Theme {
    Theme::custom(theme::Palette {
      background: color!(0x002b36),
      text: color!(0xfdf6e3),
      ..Theme::Dark.palette()
    })
  }

  fn view(&self) -> Element<Message> {
    match self {
      SingleModeShortcuts::Loaded(state @ State { input_value, .. }) => {
        let input = text_input("Enter shortcut", input_value, Message::InputChanged)
          .id(INPUT_ID.clone())
          .padding(8)
          .size(24)
          .on_submit(Message::CreateTask);

        let content = column![
          input, // maps,
          state.maps()
        ]
        .spacing(8);

        container(content)
          .width(Length::Fill)
          .padding(8)
          .center_x()
          .into()
      }
      SingleModeShortcuts::Loading => container(
        text("Loading...")
          .horizontal_alignment(alignment::Horizontal::Center)
          .size(50),
      )
      .width(Length::Fill)
      .height(Length::Fill)
      .center_y()
      .into(),
    }
  }

  fn subscription(&self) -> Subscription<Message> {
    subscription::events_with(|event, status| match (event, status) {
      (
        Event::Keyboard(keyboard::Event::KeyPressed {
          key_code: keyboard::KeyCode::Tab,
          modifiers,
          ..
        }),
        event::Status::Ignored,
      ) => Some(Message::TabPressed {
        shift: modifiers.shift(),
      }),
      _ => None,
    })
  }
}

#[derive(Debug, Clone)]
pub enum TaskState {
  Idle,
  Editing,
}

impl Default for TaskState {
  fn default() -> Self {
    Self::Idle
  }
}

#[derive(Debug, Clone)]
pub enum TaskMessage {
  Completed(bool),
  Edit,
  DescriptionEdited(String),
  FinishEdition,
  Delete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Filter {
  All,
  Active,
  Completed,
}

impl Default for Filter {
  fn default() -> Self {
    Filter::All
  }
}

impl State {
  async fn load() -> Self {
    State::new()
  }
}
