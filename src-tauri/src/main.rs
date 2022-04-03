#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{Manager, WindowUrl};

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      // `main` here is the window label; it is defined on the window creation or under `tauri.conf.json`
      // the default value is `main`. note that it must be unique
      let main_window = app.get_window("main").unwrap();
      let video_player_window = app
        .create_window(
          "video-player".to_string(),
          WindowUrl::App("https://netflix.com".into()),
          move |window_builder, webview_attributes| (window_builder, webview_attributes),
        )
        .unwrap();

      // listen to the `event-name` (emitted on the `main` window)
      main_window.listen("event-name", move |event| {
        println!("got window event-name with payload",);
        video_player_window.eval("location.href='https://google.com'");
      });
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
