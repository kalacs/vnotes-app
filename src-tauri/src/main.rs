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
        const button = document.createElement('button');
        const span = document.createElement('span');
        span.textContent = 'Go Back';
        button.title = 'Go back';
        button.id = 'go-back';
        button.appendChild(span);
        button.style.position = 'fixed';
        button.style.bottom = '20px';
        button.style.left = '20px';
        button.style.borderRadius = '45px';
        button.style.border = '1px solid transparent';
        button.style.padding = '22px 0';
        button.onclick = function() {
          window.emit('switch-to-main','video-player');
        };
        document.querySelector('body').appendChild(button);
        clearInterval(window.__test);
      };
      window.__count++;
    }, 1000);

    window.uid = function uid() {
      return window.crypto.getRandomValues(new Uint32Array(1))[0];
    };
    
    window.transformCallback = function transformCallback(callback, once) {
      var identifier = window.uid();
      var prop = `_${identifier}`;
    
      Object.defineProperty(window, prop, {
        value: (result) => {
          if (once) {
            Reflect.deleteProperty(window, prop);
          }
    
          return callback && callback(result);
        },
        writable: false,
        configurable: true,
      });
    
      return identifier;
    };
    
    window.invoke = function invoke(cmd, args) {
      return new Promise((resolve, reject) => {
        const callback = window.transformCallback((e) => {
          resolve(e);
          Reflect.deleteProperty(window, error);
        }, true);
        const error = window.transformCallback((e) => {
          reject(e);
          Reflect.deleteProperty(window, callback);
        }, true);
    
        window.__TAURI_POST_MESSAGE__({
          cmd,
          callback,
          error,
          ...args,
        });
      });
    };
    
    window.invokeTauriCommand = function invokeTauriCommand(command) {
      return window.invoke("tauri", command);
    };
    
    window.emit = function emit(event, windowLabel, payload) {
      window.invokeTauriCommand({
        __tauriModule: "Event",
        message: {
          cmd: "emit",
          event,
          windowLabel,
          payload: typeof payload === "string" ? payload : JSON.stringify(payload),
        },
      });
    };    
    "#;
  window
    .create_window(
      "video-player".to_string(),
      tauri::WindowUrl::App("https://hbomax.com".into()),
      |window_builder, webview_attributes| {
        (
          window_builder.title("Video Notes - Player"),
          webview_attributes.initialization_script(script),
        )
      },
    )
    .unwrap();
  let player_window = window.get_window("video-player").unwrap();
  let main_window = window.get_window("main").unwrap();

  player_window.listen("switch-to-main", move |event| {
    main_window.set_focus().unwrap();
  });
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
