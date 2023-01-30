use crate::types::{GoToOrLaunch, KeymapEntry, Launch, Leaf};
use anyhow::Result;
use std::{
  collections::HashMap,
  process::{self, Command},
  str,
};

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
      Leaf::LaunchNoQuit(Launch { program, args, .. }) => {
        let mut program = Command::new(program);
        program.args(*args);
        program.spawn()?;
        return Ok(());
      }
      Leaf::Quit => process::exit(0),
    }
    process::exit(0);
  }
}

impl KeymapEntry {
  pub(crate) fn is_mode(&self) -> bool {
    match self {
      KeymapEntry::Leaf(_) => false,
      KeymapEntry::Node { .. } => true,
    }
  }
  pub(crate) fn get_name(&self) -> &'static str {
    match self {
      KeymapEntry::Leaf(leaf) => match leaf {
        Leaf::GoToOrLaunch(GoToOrLaunch {
          launch: Launch { name, .. },
          ..
        }) => name,
        Leaf::Launch(Launch { name, .. }) | Leaf::LaunchNoQuit(Launch { name, .. }) => name,
        Leaf::Quit => "quit",
      },
      KeymapEntry::Node { name, .. } => name,
    }
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
