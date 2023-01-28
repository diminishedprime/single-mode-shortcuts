#![windows_subsystem = "windows"]




use std::str;



use iced::alignment::{self};
use iced::event::{self, Event};
use iced::keyboard;
use iced::subscription;
use iced::theme::Theme;
use iced::widget::{self, column, container, scrollable, text, text_input};
use iced::window;
use iced::{Application, Element};
use iced::{Command, Length, Settings, Subscription};

use keymap::get_keymap;
use keymap_entry::KeymapEntry;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

mod keymap;
mod keymap_entry;

static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);

pub fn main() -> iced::Result {
  Todos::run(Settings {
    window: window::Settings {
      size: (500, 200),
      ..window::Settings::default()
    },
    ..Settings::default()
  })
}

#[derive(Debug)]
enum Todos {
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
    State {
      input_value: "".to_string(),
      keymap: get_keymap(),
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

impl Application for Todos {
  type Message = Message;
  type Theme = Theme;
  type Executor = iced::executor::Default;
  type Flags = ();

  fn new(_flags: ()) -> (Todos, Command<Message>) {
    (
      Todos::Loading,
      Command::perform(State::load(), Message::Loaded),
    )
  }

  fn title(&self) -> String {
    "Single Mode Shortcuts".to_string()
  }

  fn update(&mut self, message: Message) -> Command<Message> {
    match self {
      Todos::Loaded(state) => {
        let command = match message {
          Message::InputChanged(value) => {
            println!("Input changed: {}", &value);
            state.input_value = value;

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
      Todos::Loading => {
        *self = Todos::Loaded(State::new());
        text_input::focus(INPUT_ID.clone())
      }
    }
  }

  fn view(&self) -> Element<Message> {
    match self {
      Todos::Loaded(State {
        input_value,
        keymap,
      }) => {
        let input = text_input("Enter shortcut", input_value, Message::InputChanged)
          .id(INPUT_ID.clone())
          .padding(15)
          .size(30)
          .on_submit(Message::CreateTask);

        let current_map = input_value
          .chars()
          .fold(Some(keymap), |acc, key| match acc {
            Some(current_map) => match current_map {
              KeymapEntry::Leaf { .. } => None,
              KeymapEntry::Node { map, .. } => map.get(&*key.to_string()),
            },
            None => None,
          });
        let maps = text(
          current_map
            .map(|map| format!("{map}"))
            .unwrap_or("No matching map.".to_string()),
        )
        .width(Length::Fill)
        .size(16);

        let content = column![input, maps].spacing(20).max_width(800);

        scrollable(
          container(content)
            .width(Length::Fill)
            .padding(40)
            .center_x(),
        )
        .into()
      }
      Todos::Loading => container(
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
