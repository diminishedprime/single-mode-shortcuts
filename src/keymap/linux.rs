use super::{keymap_for, launch, launch_no_exit};
use crate::types::{KeymapEntry, Launch, Leaf};

fn go_to_or_launch(
  workspace_name: &'static str,
  instance_match: &'static str,
  name: &'static str,
  program: &'static str,
  args: &'static [&'static str],
) -> KeymapEntry {
  use crate::types::GoToOrLaunch;

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

pub(crate) fn get_keymap() -> KeymapEntry {
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
                d8a98bef-38c4-4602-ae84-9be39fe8937e/budget"],
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
      let logout = launch("logout", "i2-msg", &["exit"]);

      keymap_for(
        "logout",
        vec![
          //
          ("x", logout),
        ],
      )
    };

    let lock = launch("lock", "mjh-lock", &[]);
    let reload = launch("reload", "i2-msg", &["reload"]);
    let restart = launch("restart", "i2-msg", &["restart"]);

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
    let bar = launch("bar", "i2-msg", &["bar mode toggle"]);
    let border = launch("border", "i2-msg", &["border toggle"]);
    // TODO - this doesn't work currently because single-mode-shortcuts is in focus when this runs.
    let floating = launch("floating", "i2-msg", &["floating toggle"]);
    let mpv_sticky_float = launch("mpv float", "i2-msg", &["[class=mpv] floating toggle, sticky toggle; [class=mpv floating] resize set 30ppt, move position 69ppt 73ppt, border none"]);

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
    let toggle_floating = launch("toggle floating", "i2-msg", &["floating toggle"]);
    // TODO - these don't really work since the window is focused. Need to figure a way around this.
    let move_to_1 = launch_no_exit(
      "move to 1",
      "i2-msg",
      &["move container to workspace 1; workspace 1"],
    );
    let move_to_0 = launch_no_exit(
      "move to 0",
      "i2-msg",
      &["move container to workspace 0; workspace 1"],
    );

    keymap_for(
      "window",
      vec![
        //
        ("f", toggle_floating),
        ("-1", move_to_0),
        ("0", move_to_1),
      ],
    )
  };

  let rofi = launch("rofi", "rofi", &["-show", "run"]);

  let framework = {
    let brightness = {
      let nighttime = launch("night", "xbacklight", &["-set", "0"]);
      let daytime = launch("day", "xbacklight", &["-set", "19"]);
      let increase = launch_no_exit("increase", "xbacklight", &["-inc", "0"]);
      let decrease = launch_no_exit("decrease", "xbacklight", &["-dec", "0"]);
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
      let full_screen_delay = launch("full screen delay", "screenshot", &["-d", "2"]);
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
