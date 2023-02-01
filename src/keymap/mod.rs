use std::collections::HashMap;

use crate::types::{KeymapEntry, Launch, Leaf};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
pub(crate) use linux::get_keymap;

#[cfg(target_os = "macos")]
pub(crate) use macos::get_keymap;

#[cfg(target_os = "windows")]
pub(crate) use windows::get_keymap;

fn keymap_for(name: &'static str, entries: Vec<(&'static str, KeymapEntry)>) -> KeymapEntry {
  let mut map: HashMap<&'static str, KeymapEntry> = HashMap::new();
  for (k, m) in entries.into_iter() {
    map.insert(k, m);
  }
  map.insert("q", KeymapEntry::Leaf(Leaf::Quit));
  KeymapEntry::Node { name, map }
}

// TODO - for things that have multiple entries, I want to somehow run the
// command, and then delete the last key.
fn launch_no_exit(
  name: &'static str,
  program: &'static str,
  args: &'static [&'static str],
) -> KeymapEntry {
  KeymapEntry::Leaf(Leaf::LaunchNoQuit(Launch {
    program,
    args,
    name,
  }))
}

fn launch(name: &'static str, program: &'static str, args: &'static [&'static str]) -> KeymapEntry {
  KeymapEntry::Leaf(Leaf::Launch(Launch {
    program,
    args,
    name,
  }))
}
