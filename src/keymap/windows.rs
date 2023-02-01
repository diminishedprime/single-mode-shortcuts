#[cfg(target_os = "windows")]
pub(crate) fn get_keymap() -> KeymapEntry {
  let apps = {
    let chrome = launch(
      "chrome",
      r"C:\Program Files\Google\Chrome\Application\chrome.exe",
      &[],
    );

    let discord = launch(
      "discord",
      r"C:\Users\matth\AppData\Local\Discord\Update.exe",
      &["--processStart", "Discord.exe"],
    );

    let vscode = launch(
      "vscode",
      r"C:\Users\matth\AppData\Local\Programs\Microsoft VS Code\Code.exe",
      &[],
    );

    keymap_for(
      "apps",
      vec![
        //
        ("c", chrome),
        ("d", discord),
        ("v", vscode),
      ],
    )
  };
  let go_to = {
    let gmail = launch(
      "gmail",
      r"C:\Program Files\Google\Chrome\Application\chrome.exe",
      &["--app=https://mail.google.com"],
    );
    let calendar = launch(
      "calendar",
      r"C:\Program Files\Google\Chrome\Application\chrome.exe",
      &["--app=https://calendar.google.com"],
    );
    let texts = launch(
      "calendar",
      r"C:\Program Files\Google\Chrome\Application\chrome.exe",
      &["--app=https://messages.google.com"],
    );
    let messenger = launch(
      "messenger",
      r"C:\Program Files\Google\Chrome\Application\chrome.exe",
      &["--app=https://messenger.com"],
    );

    keymap_for(
      "go to",
      vec![
        //
        ("g", gmail),
        ("c", calendar),
        ("t", texts),
        ("m", messenger),
      ],
    )
  };

  keymap_for(
    "",
    vec![
      //
      ("a", apps),
      ("g", go_to),
    ],
  )
}
