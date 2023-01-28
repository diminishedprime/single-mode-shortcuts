use crate::{
  keymap_entry::{GoToOrLaunch, Launch, Leaf},
  KeymapEntry,
};

use std::collections::HashMap;

fn launch(name: &'static str, program: &'static str, args: &'static [&'static str]) -> KeymapEntry {
  KeymapEntry::Leaf(Leaf::Launch(Launch {
    program,
    args,
    name,
  }))
}

fn gtol(
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

fn keymap_for(name: &'static str, entries: Vec<(&'static str, KeymapEntry)>) -> KeymapEntry {
  let mut map: HashMap<&'static str, KeymapEntry> = HashMap::new();
  for (k, m) in entries.into_iter() {
    map.insert(k, m);
  }
  map.insert("q", KeymapEntry::Leaf(Leaf::Quit));
  KeymapEntry::Node { name, map }
}

#[cfg(target_os = "linux")]
pub fn get_keymap() -> KeymapEntry {
  keymap_for(
    "",
    vec![
      (
        "a",
        keymap_for(
          "apps",
          vec![("c", launch("chrome", "google-chrome-stable", &[]))],
        ),
      ),
      (
        "g",
        keymap_for(
          "go_to",
          vec![
            ("a", gtol("", r"^anki$", "anki", "anki", &[])),
            ("d", gtol("", r"^discord$", "discord", "discord", &[])),
            ("i", gtol("", r"^signal$", "signal", "signal-desktop", &[])),
            ("s", gtol("", r"^spotify$", "spotify", "spotify", &[])),
            (
              "y",
              gtol(
                "$",
                r"^app\\.youneedabudget\\.com",
                "ynab",
                "google-chrome-stable",
                &["--app=https://app.youneedabudget.com/\
                d9a98bef-38c4-4602-ae84-9be39fe8937e/budget"],
              ),
            ),
            (
              "t",
              gtol(
                "",
                r"^messages\\.google\\.com$",
                "texts",
                "google-chrome-stable",
                &["--app=https://messages.google.com"],
              ),
            ),
            (
              "c",
              gtol(
                "",
                r"^calendar\\.google\\.com$",
                "calendar",
                "google-chrome-stable",
                &["--app=https://calendar.google.com"],
              ),
            ),
            (
              "g",
              gtol(
                "",
                r"^mail\\.google\\.com$",
                "gmail",
                "google-chrome-stable",
                &["--app=https://mail.google.com"],
              ),
            ),
            (
              "m",
              gtol(
                "",
                r"^messenger\\.com$",
                "messenger",
                "google-chrome-stable",
                &["--app=https://messenger.com"],
              ),
            ),
          ],
        ),
      ),
    ],
  )
}
