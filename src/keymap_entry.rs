use std::{
  collections::HashMap,
  fmt::Display,
  process::{self, Command},
  str,
};

use anyhow::Result;
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoToOrLaunch {
  pub(crate) workspace_name: &'static str,
  pub(crate) instance_match: &'static str,
  pub(crate) launch: Launch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Launch {
  pub(crate) name: &'static str,
  pub(crate) program: &'static str,
  pub(crate) args: &'static [&'static str],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Leaf {
  GoToOrLaunch(GoToOrLaunch),
  Launch(Launch),
  Quit,
}
impl Leaf {
  pub(crate) fn run(&self) -> Result<()> {
    match self {
      Leaf::GoToOrLaunch(GoToOrLaunch {
        workspace_name,
        instance_match,
        launch: Launch { program, args, .. },
      }) => {
        let mut save_tree = Command::new("i3-save-tree");
        save_tree.arg("--workspace").arg(workspace_name);

        let mut go_to = Command::new("i3-msg");
        go_to.arg(format!("workspace {workspace_name}"));
        go_to.spawn()?;

        if !str::from_utf8(&save_tree.output()?.stdout)?.contains(instance_match) {
          let mut program = Command::new(program);
          program.args(*args);
          program.spawn()?;
        }
      }
      Leaf::Launch(Launch { program, args, .. }) => {
        let mut program = Command::new(program);
        program.args(*args);
        program.spawn()?;
      }
      Leaf::Quit => process::exit(0),
    }
    process::exit(0);
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeymapEntry {
  Leaf(Leaf),
  Node {
    name: &'static str,
    map: HashMap<&'static str, KeymapEntry>,
  },
}

impl KeymapEntry {
  fn get_name(&self) -> &'static str {
    match self {
      KeymapEntry::Leaf(leaf) => match leaf {
        Leaf::GoToOrLaunch(GoToOrLaunch {
          launch: Launch { name, .. },
          ..
        }) => name,
        Leaf::Launch(Launch { name, .. }) => name,
        Leaf::Quit => "q",
      },
      KeymapEntry::Node { name, .. } => name,
    }
  }
}

impl PartialOrd for KeymapEntry {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    self.get_name().partial_cmp(other.get_name())
  }
}

impl Ord for KeymapEntry {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.get_name().cmp(other.get_name())
  }
}

impl Default for KeymapEntry {
  fn default() -> Self {
    Self::Node {
      name: "top",
      map: HashMap::new(),
    }
  }
}

impl KeymapEntry {
  pub(crate) fn run(&self, input_value: &str) {
    if let Some(KeymapEntry::Leaf(leaf)) =
      input_value.chars().fold(Some(self), |acc, key| match acc {
        Some(current_map) => match current_map {
          KeymapEntry::Leaf { .. } => None,
          KeymapEntry::Node { map, .. } => map.get(&*key.to_string()),
        },
        None => None,
      })
    {
      leaf.run().unwrap();
    }
  }
}

impl Display for KeymapEntry {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      KeymapEntry::Leaf(leaf) => match leaf {
        Leaf::GoToOrLaunch(GoToOrLaunch {
          launch: Launch { name, .. },
          ..
        }) => write!(f, "{name}"),
        Leaf::Launch(Launch { name, .. }) => write!(f, "{name}"),
        Leaf::Quit => write!(f, "Quit"),
      },
      KeymapEntry::Node { map, .. } => {
        for (key, value) in map.iter().sorted() {
          let formatted_value = match value {
            KeymapEntry::Leaf(leaf) => match leaf {
              Leaf::GoToOrLaunch(GoToOrLaunch {
                launch: Launch { name, .. },
                ..
              }) => name.to_string(),
              Leaf::Launch(Launch { name, .. }) => name.to_string(),
              Leaf::Quit => "Quit".to_string(),
            },
            KeymapEntry::Node { name, .. } => format!("mode-{name}"),
          };
          write!(f, "{key}:{formatted_value} ")?
        }
        Ok(())
      }
    }
  }
}
