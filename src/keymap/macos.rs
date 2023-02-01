#[cfg(target_os = "macos")]
pub(crate) fn get_keymap() -> KeymapEntry {
  let apps = {
    keymap_for(
      "apps",
      vec![
        //
      ],
    )
  };

  keymap_for("", vec![("a", apps)])
}
