use std::{
  collections::HashMap,
  fmt::Display,
  process::{self, Command},
  str,
};

use anyhow::Result;

#[derive(Debug, Clone)]
pub struct GoToOrLaunch {
  workspace_name: &'static str,
  instance_match: &'static str,
  launch: Launch,
}

#[derive(Debug, Clone)]
pub struct Launch {
  name: &'static str,
  program: &'static str,
  args: &'static [&'static str],
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum KeymapEntry {
  Leaf(Leaf),
  Node {
    name: &'static str,
    map: HashMap<&'static str, KeymapEntry>,
  },
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
  pub(crate) fn new_leaf(
    name: &'static str,
    program: &'static str,
    args: &'static [&'static str],
  ) -> KeymapEntry {
    KeymapEntry::Leaf(Leaf::Launch(Launch {
      program,
      args,
      name,
    }))
  }

  pub(crate) fn go_to_or_launch(
    workspace_name: &'static str,
    instance_match: &'static str,
    name: &'static str,
    program: &'static str,
    args: &'static [&'static str],
  ) -> KeymapEntry {
    KeymapEntry::Leaf(Leaf::GoToOrLaunch(GoToOrLaunch {
      workspace_name,
      instance_match,
      launch: Launch {
        name,
        program,
        args,
      },
    }))
  }

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
        }) => write!(f, "Launch: {name}"),
        Leaf::Launch(Launch { name, .. }) => write!(f, "Launch: {name}"),
        Leaf::Quit => write!(f, "Quit"),
      },
      KeymapEntry::Node { map, .. } => {
        for (key, value) in map.iter() {
          let formatted_value = match value {
            KeymapEntry::Leaf(leaf) => match leaf {
              Leaf::GoToOrLaunch(GoToOrLaunch {
                launch: Launch { name, .. },
                ..
              }) => format!("Launch: {name}"),
              Leaf::Launch(Launch { name, .. }) => format!("Launch: {name}"),
              Leaf::Quit => format!("Quit"),
            },
            KeymapEntry::Node { name, .. } => format!("m:{name}"),
          };
          write!(f, " |{key}->{formatted_value}|")?
        }
        Ok(())
      }
    }
  }
}
