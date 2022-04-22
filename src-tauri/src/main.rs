#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{Manager, WindowBuilder, WindowEvent};

#[tauri::command]
async fn open_window(mut window: tauri::Window) {
  let script = r#"
    window.__count = 0;
    window.__test = setInterval(() => {
      const body = document.querySelector('body');

      if (body || window.__count < 2 ) {
        window.__count = 0;
        const element = document.createElement('h1');
        element.textContent = 'ADFASDFSADFAddSDFASDF';
        document.querySelector('body').appendChild(element);
        clearInterval(window.__test);
      };
      window.__count++;
    }, 1000);
    "#;
  window
    .create_window(
      "video-player".to_string(),
      tauri::WindowUrl::App("https://hbomax.com".into()),
      |window_builder, webview_attributes| {
        (
          window_builder
            .title("Video Notes - Player")
            .always_on_top(false),
          webview_attributes.initialization_script(script),
        )
      },
    )
    .unwrap();
  window.get_window("main").unwrap().set_focus().unwrap();
}

fn main() {
  tauri::Builder::default()
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
    .invoke_handler(tauri::generate_handler![open_window])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
