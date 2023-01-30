use crate::{
  keymap::get_keymap,
  types::{Args, KeymapEntry, Leaf, Message, SingleModeShortcuts, State},
  INPUT_ID, ROBOTO,
};
use clap::Parser;
use iced::{
  alignment, color, theme,
  widget::{column, container, row, text, text_input},
  window, Application, Command, Element, Font, Length, Settings, Theme,
};

pub(crate) fn main() -> iced::Result {
  SingleModeShortcuts::run(Settings {
    window: window::Settings {
      size: (500, 300),
      decorations: false,
      ..window::Settings::default()
    },
    ..Settings::default()
  })
}

impl State {
  pub(crate) fn new() -> State {
    let args = Args::parse();
    State {
      input_value: args.input,
      keymap: get_keymap(),
    }
  }
  async fn load() -> Self {
    Self::new()
  }
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

            if !matches!(current_map, Some(KeymapEntry::Leaf(Leaf::LaunchNoQuit(_)))) {
              state.input_value = value;
            }

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
      SingleModeShortcuts::Loaded(
        _state @ State {
          input_value,
          keymap,
        },
      ) => {
        let input = text_input("Enter shortcut", input_value, Message::InputChanged)
          .id(INPUT_ID.clone())
          .padding(8)
          .size(24);

        let content = column![
          input, // maps,
          keymap.view(input_value)
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
}

impl KeymapEntry {
  pub(crate) fn view(&self, input: &str) -> Element<Message> {
    let current_map = input.chars().fold(Some(self), |acc, key| match acc {
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
