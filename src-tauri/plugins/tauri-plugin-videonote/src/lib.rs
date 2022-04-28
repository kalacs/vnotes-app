use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime, AppHandle,
  window::WindowBuilder,
};

#[tauri::command]
async fn open_window<R: Runtime>(app: AppHandle<R>) {
  let script = r#"
  console.log("INJECTED")
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
          window.invoke('plugin:videonote|switch_to_main');
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
    WindowBuilder::new(
      &app,
      "video-player".to_string(),
      tauri::WindowUrl::App("https://hbomax.com".into())
    )
      .title("Video Notes - Player")
      .initialization_script(script).build();
}

#[tauri::command]
async fn switch_to_main<R: Runtime>(app: AppHandle<R>){
  let main = app.get_window("main").unwrap();
  main.set_focus();
}

#[tauri::command]
async fn switch_to_provider<R: Runtime>(app: AppHandle<R>){
  let provider = app.get_window("video-player").unwrap();
  provider.set_focus();
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("videonote")
    .invoke_handler(tauri::generate_handler![open_window, switch_to_main, switch_to_provider])
    .build()
}
