use crate::KeymapEntry;
use std::collections::HashMap;

fn keymap_for(name: &'static str, entries: Vec<(&'static str, KeymapEntry)>) -> KeymapEntry {
  let mut map: HashMap<&'static str, KeymapEntry> = HashMap::new();
  for (k, m) in entries.into_iter() {
    map.insert(k, m);
  }
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
          vec![(
            "c",
            KeymapEntry::new_leaf("chrome", "google-chrome-stable", &[]),
          )],
        ),
      ),
      (
        "g",
        keymap_for(
          "go_to",
          vec![
            (
              "g",
              KeymapEntry::go_to_or_launch(
                "",
                r"^mail\\.google\\.com$",
                "gmail",
                "google-chrome-stable",
                &["--app=https://mail.google.com"],
              ),
            ),
            (
              "m",
              KeymapEntry::go_to_or_launch(
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
