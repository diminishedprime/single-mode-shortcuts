use crate::{
  keymap_entry::{GoToOrLaunch, Launch, Leaf},
  KeymapEntry,
};

use std::collections::HashMap;

// TODO - for things that have multiple entries, I want to somehow run the
// command, and then delete the last key.
fn launch_no_exit(
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

fn launch(name: &'static str, program: &'static str, args: &'static [&'static str]) -> KeymapEntry {
  KeymapEntry::Leaf(Leaf::Launch(Launch {
    program,
    args,
    name,
  }))
}

fn go_to_or_launch(
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
  let apps = {
    let chrome = launch("chrome", "google-chrome-stable", &[]);
    let files = launch("files", "thunar", &[]);

    keymap_for(
      "apps",
      vec![
        //
        ("c", chrome),
        ("f", files),
      ],
    )
  };

  let go_to = {
    let anki = go_to_or_launch("", r"^anki$", "anki", "anki", &[]);
    let discord = go_to_or_launch("", r"^discord$", "discord", "discord", &[]);
    let signal = go_to_or_launch("", r"^signal$", "signal", "signal-desktop", &[]);
    let spotify = go_to_or_launch("", r"^spotify$", "spotify", "spotify", &[]);
    let ynab = go_to_or_launch(
      "$",
      r"^app\\.youneedabudget\\.com",
      "ynab",
      "google-chrome-stable",
      &["--app=https://app.youneedabudget.com/\
                d9a98bef-38c4-4602-ae84-9be39fe8937e/budget"],
    );
    let texts = go_to_or_launch(
      "",
      r"^messages\\.google\\.com$",
      "texts",
      "google-chrome-stable",
      &["--app=https://messages.google.com"],
    );
    let calendar = go_to_or_launch(
      "",
      r"^calendar\\.google\\.com$",
      "calendar",
      "google-chrome-stable",
      &["--app=https://calendar.google.com"],
    );
    let gmail = go_to_or_launch(
      "",
      r"^mail\\.google\\.com$",
      "gmail",
      "google-chrome-stable",
      &["--app=https://mail.google.com"],
    );
    let messenger = go_to_or_launch(
      "",
      r"^messenger\\.com$",
      "messenger",
      "google-chrome-stable",
      &["--app=https://messenger.com"],
    );

    keymap_for(
      "go_to",
      vec![
        //
        ("a", anki),
        ("d", discord),
        ("i", signal),
        ("s", spotify),
        ("y", ynab),
        ("t", texts),
        ("c", calendar),
        ("g", gmail),
        ("m", messenger),
      ],
    )
  };

  let admin = {
    let logout = {
      let logout = launch("logout", "i3-msg", &["exit"]);

      keymap_for(
        "logout",
        vec![
          //
          ("x", logout),
        ],
      )
    };

    let lock = launch("lock", "mjh-lock", &[]);
    let reload = launch("reload", "i3-msg", &["reload"]);
    let restart = launch("restart", "i3-msg", &["restart"]);

    keymap_for(
      "admin",
      vec![
        //
        ("l", lock),
        ("e", reload),
        ("r", restart),
        ("x", logout),
      ],
    )
  };

  let sound = {
    let alsamixer = launch(
      "mixer",
      "alacritty",
      &["--class", "float", "-e", "pulsemixer"],
    );
    keymap_for(
      "sound",
      vec![
        //
        ("a", alsamixer),
      ],
    )
  };

  let toggle = {
    let bar = launch("bar", "i3-msg", &["bar mode toggle"]);
    let border = launch("border", "i3-msg", &["border toggle"]);
    // TODO - this doesn't work currently because single-mode-shortcuts is in focus when this runs.
    let floating = launch("floating", "i3-msg", &["floating toggle"]);
    let mpv_sticky_float = launch("mpv float", "i3-msg", &["[class=mpv] floating toggle, sticky toggle; [class=mpv floating] resize set 30ppt, move position 69ppt 73ppt, border none"]);

    keymap_for(
      "toggle",
      vec![
        //
        ("b", bar),
        ("B", border),
        ("f", floating),
        ("s", mpv_sticky_float),
      ],
    )
  };

  let window = {
    //
    keymap_for("window", vec![])
  };

  let rofi = launch("rofi", "rofi", &["-show", "run"]);

  let framework = {
    let brightness = {
      let nighttime = launch("night", "xbacklight", &["-set", "1"]);
      let daytime = launch("day", "xbacklight", &["-set", "20"]);
      let increase = launch_no_exit("increase", "xbacklight", &["-inc", "1"]);
      let decrease = launch_no_exit("decrease", "xbacklight", &["-dec", "1"]);
      keymap_for(
        "brightness",
        vec![
          //
          ("n", nighttime),
          ("d", daytime),
          ("k", increase),
          ("j", decrease),
        ],
      )
    };

    let screenshots = {
      let full_screen = launch("full screen", "screenshot", &[]);
      let select_area = launch("select area", "screenshot", &["-sD"]);
      let full_screen_delay = launch("full screen delay", "screenshot", &["-d", "3"]);
      let current_window = launch("current window", "screenshot", &["-s"]);

      keymap_for(
        "screenshots",
        vec![
          //
          ("u", full_screen),
          ("a", select_area),
          ("d", full_screen_delay),
          ("c", current_window),
        ],
      )
    };
    keymap_for(
      "framework",
      vec![
        //
        ("b", brightness),
        ("s", screenshots),
      ],
    )
  };

  keymap_for(
    "",
    vec![
      ("a", apps),
      ("g", go_to),
      ("i", admin),
      ("s", sound),
      ("c", toggle),
      ("w", window),
      (" ", rofi),
      ("f", framework),
    ],
  )
}
