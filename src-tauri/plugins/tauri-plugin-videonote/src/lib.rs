use std::{
    fs::File,
    io::{Error, Read},
};
use tauri::{
    plugin::{Builder, TauriPlugin},
    window::WindowBuilder,
    AppHandle, Manager, Runtime,
};

#[derive(Clone, serde::Serialize)]
struct Payload {
    name: String,
    data: String,
}

fn read_script_from_file(filename: &str) -> Result<String, Error> {
    let mut content = String::new();
    File::open(filename)?.read_to_string(&mut content)?;
    Ok(content)
}

#[tauri::command]
async fn open_window<R: Runtime>(app: AppHandle<R>) {
    let result = read_script_from_file("plugins/tauri-plugin-videonote/js/index.js").unwrap();
    WindowBuilder::new(
        &app,
        "video-player".to_string(),
        tauri::WindowUrl::App("https://hbomax.com".into()),
    )
    .title("Video Notes - Player")
    .initialization_script(&result)
    .build()
    .unwrap();
}

#[tauri::command]
async fn switch_to_main<R: Runtime>(app: AppHandle<R>) {
    let main = app.get_window("main").unwrap();
    main.set_focus().unwrap();
}

#[tauri::command]
async fn switch_to_provider<R: Runtime>(app: AppHandle<R>) {
    let provider = app.get_window("video-player").unwrap();
    provider.set_focus().unwrap();
}

#[tauri::command]
async fn play_content<R: Runtime>(app: AppHandle<R>) {
    let provider = app.get_window("video-player").unwrap();
    provider.eval("window.__videoPlayer.play()").unwrap();
}

#[tauri::command]
async fn pause_content<R: Runtime>(app: AppHandle<R>) {
    let provider = app.get_window("video-player").unwrap();
    provider.eval("window.__videoPlayer.pause()").unwrap();
}

#[tauri::command]
async fn seek_content<R: Runtime>(app: AppHandle<R>, time: u16) {
    let provider = app.get_window("video-player").unwrap();
    println!("TIME: {:?}", time);
    provider
        .eval(format!("window.__videoPlayer.fastSeek({})", time).as_str())
        .unwrap();
}

#[tauri::command]
async fn connect_player<R: Runtime>(app: AppHandle<R>) {
    let provider = app.get_window("video-player").unwrap();
    provider.eval("window.__findVideoPlayer();").unwrap();
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("videonote")
        .setup(|app| {
            let app_copy = app.clone();
            let app_copy_two = app.clone();
            app.listen_global("videonotes://video-player-found", move |_event| {
                let main_window = app_copy.get_window("main");
                main_window
                    .unwrap()
                    .emit("videonotes://video-player-found", "")
                    .unwrap();
            });
            app.listen_global("videonotes://video-player-event", move |event| {
                let main_window = app_copy_two.get_window("main");
                println!("got event-name with payload {:?}", event.payload());
                main_window
                    .unwrap()
                    .emit(
                        "videonotes://video-player-event",
                        Payload {
                            name: "Tauri is awesome!".into(),
                            data: event.payload().unwrap().to_string(),
                        },
                    )
                    .unwrap();
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            open_window,
            switch_to_main,
            switch_to_provider,
            play_content,
            pause_content,
            seek_content,
            connect_player
        ])
        .build()
}
