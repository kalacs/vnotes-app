use core::num;
use serde::{Deserialize, Serialize};
use serde_json::json;
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
struct PluginState {
    current_time: Mutex<f32>,
    loaded_notes: Mutex<Vec<VideoNote>>,
}
#[derive(Clone, Serialize)]
struct Payload {
    name: String,
    payload: VideoEvent,
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

#[derive(Clone, Deserialize, Serialize, Debug)]
struct VideoEvent {
    name: String,
    currentTime: f32,
    data: Option<VideoNote>,
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
    .maximized(true)
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
async fn load_notes<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, PluginState>,
) -> Result<(), ()> {
    let resp = reqwest::get("http://127.0.0.1:3000").await.unwrap();
    let data = resp.json::<Vec<VideoNote>>().await;
    *state.loaded_notes.lock().unwrap() = data.unwrap();
    println!("LOADED!");
    app.get_window("main")
        .unwrap()
        .emit("videonotes://notes-loaded", "")
        .unwrap();
    Ok(())
}

fn round(number: f32) -> f32 {
    /*
      let fraction = (format!("{:.1}", number).parse::<f32>().unwrap().fract() * 10.0).round();
        let trunk: f32 = number.trunc();
        println!(
            "FRACTION: {} --> FORMAT: {}",
            fraction,
            format!("{:.1}", number)
        );
        let result = match fraction {
            6.0 | 7.0 | 8.0 | 9.0 => trunk + 1.0,
            0.0 => trunk,
            _ => trunk + 0.5,
        };
        result
    */
    format!("{:.1}", number).parse::<f32>().unwrap()
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("videonote")
        .setup(|app| {
            app.manage(PluginState {
                current_time: Mutex::new(0.0),
                loaded_notes: Mutex::new(Vec::<VideoNote>::with_capacity(1000)),
            });

            let app_copy = app.clone();
            app.listen_global("videonotes://video-player-found", move |_event| {
                let main_window = app_copy.get_window("main");
                main_window
                    .unwrap()
                    .emit("videonotes://video-player-found", "")
                    .unwrap();
            });
            let app_copy = app.clone();
            app.listen_global("videonotes://video-player-event", move |event| {
                let main_window = app_copy.get_window("main");
                let video_event: VideoEvent =
                    serde_json::from_str(&event.payload().unwrap().to_string()).unwrap();
                let video_event_name = video_event.name.to_string();
                let state: State<'_, PluginState> = app_copy.state();
                let video_notes: &Vec<VideoNote> = &*state.loaded_notes.lock().unwrap();
                let mut new_video_event = video_event.clone();
                let offset = 0.3;

                if video_event_name == "timeupdate" {
                    let mut index = 0;
                    let video_note_result: Option<VideoNote> = loop {
                        if index + 1 == video_notes.len() {
                            break None;
                        }
                        let video_note: &VideoNote = &video_notes[index];
                        index = index + 1;
                        let start_time_rounded: f32 = round(video_note.startTime);
                        let current_time_rounded: f32 = round(video_event.currentTime);
                        let lower_bound_rounded: f32 = round(current_time_rounded - offset);
                        // currentTime - 0.20 <= startTime <= currentTime
                        if lower_bound_rounded <= start_time_rounded
                            && start_time_rounded <= current_time_rounded
                        {
                            break Some(video_notes[if index > 0 { index - 1 } else { 0 }].clone());
                        }
                    };

                    match video_note_result {
                        Some(video_note) => {
                            new_video_event.data = Some(video_note);
                            main_window
                                .unwrap()
                                .emit(
                                    "videonotes://video-player-event",
                                    Payload {
                                        name: "cue".to_string(),
                                        payload: new_video_event,
                                    },
                                )
                                .unwrap();
                        }
                        None => (),
                    }
                } else {
                    main_window
                        .unwrap()
                        .emit(
                            "videonotes://video-player-event",
                            Payload {
                                name: video_event_name,
                                payload: video_event,
                            },
                        )
                        .unwrap();
                }
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
        ])
        .build()
}
