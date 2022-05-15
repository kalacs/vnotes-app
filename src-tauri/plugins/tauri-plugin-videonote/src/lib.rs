use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Error, Read},
    sync::Mutex,
};
use tauri::{
    plugin::{Builder, TauriPlugin},
    window::WindowBuilder,
    AppHandle, Manager, Runtime, State,
};
struct LoadedNotes(Mutex<Vec<VideoNote>>);

#[derive(Clone, Serialize)]
struct Payload {
    name: String,
    data: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
struct VideoNote {
    startTime: f32,
    endTime: f32,
    payload: VideoNotePayload,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
struct VideoNotePayload {
    content: String,
    r#type: String,
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
    .decorations(false)
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
        .eval(format!("window.__videoPlayer.currentTime = {}", time).as_str())
        .unwrap();
}

#[tauri::command]
async fn connect_player<R: Runtime>(app: AppHandle<R>) {
    let provider = app.get_window("video-player").unwrap();
    provider.eval("window.__findVideoPlayer();").unwrap();
}

#[tauri::command]
fn get_state(state: State<'_, LoadedNotes>) -> Vec<VideoNote> {
    let data = &*state.0.lock().unwrap();
    let test = data.clone();
    test
}

#[tauri::command]
async fn load_notes<R: Runtime>(
    app: AppHandle<R>,
    loadedNotes: State<'_, LoadedNotes>,
) -> Result<(), ()> {
    let resp = reqwest::get("http://127.0.0.1:3000").await.unwrap();
    let data = resp.json::<Vec<VideoNote>>().await;
    let mut mutex_data = loadedNotes.0.lock().unwrap();
    *mutex_data = data.unwrap();
    app.get_window("main")
        .unwrap()
        .emit("videonotes://notes-loaded", "")
        .unwrap();
    Ok(())
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("videonote")
        .setup(|app| {
            let app_copy = app.clone();
            let app_copy_two = app.clone();
            app.manage(LoadedNotes(Mutex::new(Vec::<VideoNote>::with_capacity(
                1000,
            ))));
            app.listen_global("videonotes://video-player-found", move |_event| {
                let main_window = app_copy.get_window("main");
                main_window
                    .unwrap()
                    .emit("videonotes://video-player-found", "")
                    .unwrap();
            });
            app.listen_global("videonotes://video-player-event", move |event| {
                let main_window = app_copy_two.get_window("main");
                println!("EVENT: {:?}", event);
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
            connect_player,
            load_notes,
            get_state,
        ])
        .build()
}
