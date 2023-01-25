use iced::alignment::{self, Alignment};
use iced::event::{self, Event};
use iced::keyboard;
use iced::subscription;
use iced::theme::{self, Theme};
use iced::widget::{
    self, button, checkbox, column, container, row, scrollable, text, text_input, Text,
};
use iced::window;
use iced::{Application, Element};
use iced::{Color, Command, Font, Length, Settings, Subscription};

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
    Loaded(State),
}

#[derive(Debug, Default)]
struct State {
    input_value: String,
}

#[derive(Debug, Clone)]
enum Message {
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
            Todos::Loaded(State {
                input_value: "".to_string(),
            }),
            Command::none(),
        )
    }

    fn title(&self) -> String {
        format!("Single Mode Shortcuts")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            Todos::Loaded(state) => {
                let mut saved = false;

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
                        println!(
                            "This is where I would call an external command and then quit., {:?}",
                            state.input_value
                        );
                        Command::none()
                    }
                    _ => Command::none(),
                };

                Command::batch(vec![command])
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match self {
            Todos::Loaded(State { input_value, .. }) => {
                let input = text_input("Enter shortcut", input_value, Message::InputChanged)
                    .id(INPUT_ID.clone())
                    .padding(15)
                    .size(30)
                    .on_submit(Message::CreateTask);

                let content = column![input].spacing(20).max_width(800);

                scrollable(
                    container(content)
                        .width(Length::Fill)
                        .padding(40)
                        .center_x(),
                )
                .into()
            }
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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    description: String,
    completed: bool,

    #[serde(skip)]
    state: TaskState,
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

// Persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SavedState {
    input_value: String,
    filter: Filter,
    tasks: Vec<Task>,
}
