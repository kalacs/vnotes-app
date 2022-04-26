#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{Manager, WindowEvent};

fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_videonote::init())
    .setup(|app| {
      // `main` here is the window label; it is defined on the window creation or under `tauri.conf.json`
      // the default value is `main`. note that it must be unique
      let main_window = app.get_window("main").unwrap();

      // listen to the `event-name` (emitted on the `main` window)
      main_window.listen("switch-to-provider", |event| {
        //  main_window.get_window("video-player");
      });
      Ok(())
    })
    .on_window_event(|event| {
      let video_player_window_label = "video-player";
      let window_label = event.window().label();
      let i = event.event();
      println!("{:?}", i);
      // in case of focus = true and window.label = video-player, inject script
      match i {
        WindowEvent::Focused(true) => {
          if window_label == video_player_window_label {
            println!("{:?}", event.window().label());
          }
        }
        _ => {}
      }
    })
    .on_page_load(|window, _payload| {
      println!("PAGE LOADED {:?}", window.label());
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
