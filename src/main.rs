#![windows_subsystem = "windows"]
use std::collections::HashMap;
use std::fmt::Display;

use iced::alignment::{self};
use iced::event::{self, Event};
use iced::keyboard;
use iced::subscription;
use iced::theme::Theme;
use iced::widget::{self, column, container, scrollable, text, text_input};
use iced::window;
use iced::{Application, Element};
use iced::{Command, Length, Settings, Subscription};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

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
            keymap: {
                let mut go_to = HashMap::new();
                go_to.insert(
                    "g".to_string(),
                    KeymapEntry::Leaf(
                        "gmail".to_string(),
                        vec![
                            r#"C:\Program Files\Google\Chrome\Application\chrome.exe"#.to_string(),
                            "--app".to_string(),
                            "https://mail.google.com/".to_string(),
                        ],
                    ),
                );
                go_to.insert(
                    "m".to_string(),
                    KeymapEntry::Leaf(
                        "fb messenger".to_string(),
                        vec![
                            r#"C:\Program Files\Google\Chrome\Application\chrome.exe"#.to_string(),
                            "--app".to_string(),
                            "https://messenger.com/".to_string(),
                        ],
                    ),
                );
                go_to.insert(
                    "t".to_string(),
                    KeymapEntry::Leaf(
                        "texts".to_string(),
                        vec![
                            r#"C:\Program Files\Google\Chrome\Application\chrome.exe"#.to_string(),
                            "--app".to_string(),
                            "https://messages.google.com/".to_string(),
                        ],
                    ),
                );

                let mut top_level = HashMap::new();
                top_level.insert(
                    "g".to_string(),
                    KeymapEntry::Node {
                        name: "go_to".to_string(),
                        map: go_to,
                    },
                );
                KeymapEntry::Node {
                    name: "top".to_string(),
                    map: top_level,
                }
            },
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
        format!("Single Mode Shortcuts")
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
                            KeymapEntry::Leaf(_, _) => None,
                            KeymapEntry::Node { map, .. } => map.get(&key.to_string()),
                        },
                        None => None,
                    });
                let maps = text(
                    current_map
                        .map(|map| format!("{}", map))
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

#[derive(Debug, Clone)]
enum KeymapEntry {
    Leaf(String, Vec<String>),
    Node {
        name: String,
        map: HashMap<String, KeymapEntry>,
    },
}

impl Default for KeymapEntry {
    fn default() -> Self {
        Self::Node {
            name: "top".to_string(),
            map: HashMap::new(),
        }
    }
}

impl KeymapEntry {
    fn run(&self, input_value: &str) {
        match input_value.chars().fold(Some(self), |acc, key| match acc {
            Some(current_map) => match current_map {
                KeymapEntry::Leaf(_, _) => None,
                KeymapEntry::Node { map, .. } => map.get(&key.to_string()),
            },
            None => None,
        }) {
            Some(KeymapEntry::Leaf(command, args)) => {
                println!("Running {}", command);
                let (name, args) = args.split_at(1);
                std::process::Command::new(name[0].clone())
                    .args(args)
                    .spawn()
                    .unwrap();
                println!("Exiting.");
                std::process::exit(0)
            }
            _ => (),
        }
    }
}

impl Display for KeymapEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeymapEntry::Leaf(leaf, _) => write!(f, "Launch: {leaf}", leaf = leaf),
            KeymapEntry::Node { map, .. } => {
                for (key, value) in map.iter() {
                    let formatted_value = match value {
                        KeymapEntry::Leaf(leaf, _) => format!("{leaf}", leaf = leaf),
                        KeymapEntry::Node { name, .. } => format!("m:{name}", name = name),
                    };
                    write!(f, " |{key}->{value}|", key = key, value = formatted_value)?
                }
                Ok(())
            }
        }
    }
}
