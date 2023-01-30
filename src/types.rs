use clap::Parser;
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about= None)]
pub(crate) struct Args {
  /// Initial state to the input. For example "g" will go straight to the go_to mode.
  #[arg(short, long, default_value_t = String::new())]
  pub(crate) input: String,
}

pub(crate) enum SingleModeShortcuts {
  Loading,
  Loaded(State),
}

#[derive(Default, Debug, Clone)]
pub(crate) struct State {
  pub(crate) input_value: String,
  pub(crate) keymap: KeymapEntry,
}

#[derive(Clone, Debug)]
pub(crate) struct GoToOrLaunch {
  pub(crate) workspace_name: &'static str,
  pub(crate) instance_match: &'static str,
  pub(crate) launch: Launch,
}

#[derive(Clone, Debug)]
pub(crate) struct Launch {
  pub(crate) name: &'static str,
  pub(crate) program: &'static str,
  pub(crate) args: &'static [&'static str],
}

#[derive(Clone, Debug)]
pub(crate) enum Leaf {
  GoToOrLaunch(GoToOrLaunch),
  Launch(Launch),
  LaunchNoQuit(Launch),
  Quit,
}

#[derive(Clone, Debug)]
pub(crate) enum KeymapEntry {
  Leaf(Leaf),
  Node {
    name: &'static str,
    map: HashMap<&'static str, KeymapEntry>,
  },
}

#[derive(Clone, Debug)]
pub(crate) enum Message {
  Loaded(State),
  InputChanged(String),
}
