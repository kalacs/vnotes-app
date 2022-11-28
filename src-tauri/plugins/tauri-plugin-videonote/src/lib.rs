use serde::{Deserialize, Serialize};
use serde_json::{Value, Map};
use std::{
    fs::File,
    io::{BufRead, BufReader, Error, Read},
    sync::Mutex,
};
use tauri::{
    api::path::download_dir,
    plugin::{Builder, TauriPlugin},
    window::WindowBuilder,
    AppHandle, Manager, Runtime, State,
};
struct PluginState {
    loaded_notes: Mutex<Vec<VideoNote>>,
    end_notes: Mutex<Vec<VideoNoteEnd>>,
    chapters: Mutex<Vec<VideoChapter>>,
}
#[derive(Clone, Serialize)]
struct Payload {
    name: String,
    payload: VideoEvent,
}
#[derive(Clone, Serialize)]
struct PayloadForEndEvent {
    name: String,
    payload: VideoEventEnd,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
struct VideoNote {
    id: i32,
    start: f32,
    end: f32,
    payload: VideoNotePayload,
}

trait TransformVideoNotes {
    fn to_html(&self, _chapters: Vec<VideoChapter>) -> String {
        String::from("")
    }
}

fn get_video_notes_between_time(video_notes: Vec<VideoNote>, start: f32, end: f32) -> Vec<VideoNote>{
    video_notes.iter().filter(|video_note| video_note.start >= start && video_note.end <= end).map(|x| x.clone()).collect::<Vec<VideoNote>>()
}

fn video_note_to_html(video_note: &VideoNote) -> String {
    let content = video_note.payload.content.replace("\n", "<br />");
    let mut references_content: String = String::from("");

    if let Some(references) = &video_note.payload.references {
      references_content = references.iter()
        .map(|reference:&VideoNoteReference| format!("{}::{};", reference.id, reference.phrase))
        .reduce(|mut accum: String, item: String| {
            accum.push_str(&item.to_string());
            accum
        })
        .unwrap()
    }

    return format!(
        "<video-note type=\"{}\" start=\"{}\" end=\"{}\" references=\"{}\" id={}>{}</video-note>",
        video_note.payload.r#type, video_note.start, video_note.end, references_content, video_note.id,content
    );
}

impl TransformVideoNotes for Vec<VideoNote> {
    fn to_html(&self, chapters: Vec<VideoChapter>) -> String {
        if chapters.len() > 0 {
            // get video notes of a chapter
            return chapters.iter().map(|chapter: &VideoChapter| {
                // iterate chapters
                let test = self.clone();
                let mut test2 = get_video_notes_between_time(test, chapter.start, chapter.end);
                let video_notes_content = test2
                    .iter_mut()
                    .map(|video_note| {
                        video_note_to_html(video_note)
                    })
                    .reduce(|mut accum: String, item: String| {
                        accum.push_str(&item.to_string());
                        accum
                    })
                    .unwrap();
                format!("<chapter id=\"{}\" title=\"{}\" start=\"{}\" end=\"{}\">{}</chapter>", chapter.id, chapter.title, chapter.start, chapter.end, video_notes_content)
            }).reduce(|mut accum: String, item: String| {
                accum.push_str(&item.to_string());
                accum
            })
            .unwrap()
        } else {
            // get video notes of a chapter
            let mut test = self.clone();
            return test
                .iter_mut()
                .map(|video_note| {
                    video_note_to_html(video_note)
                })
                .reduce(|mut accum: String, item: String| {
                    accum.push_str(&item.to_string());
                    accum
                })
                .unwrap();
        }

    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
struct VideoNoteEnd {
    action_time: f32,
    payload: VideoNotePayload,
    id: i32,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
struct VideoChapter {
    id: i32,
    title: String,
    start: f32,
    end: f32,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
struct VideoNoteReference {
    id: i32,
    phrase: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
struct VideoNotePayload {
    content: String,
    r#type: String,
    references: Option<Vec<VideoNoteReference>>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
struct VideoEvent {
    name: String,
    time: f32,
    data: Option<VideoNote>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
struct VideoEventEnd {
    name: String,
    time: f32,
    data: Option<VideoNoteEnd>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
struct EditorContent {
    content: Vec<Value>,
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

fn parse_time_string_to_float(time_string: &str) -> f32 {
    let mut tokens: Vec<&str> = time_string.split(&[':', ','][..]).collect();
    let time_fragment = tokens.pop().unwrap();
    let ints: Vec<i32> = tokens
        .iter_mut()
        .map(|token| token.parse::<i32>().unwrap())
        .collect();
    let main = ((ints[0] * 60 * 60) + (ints[1] * 60) + ints[2]) as f32;
    let rest: f32 = format!("0.{}", time_fragment).parse::<f32>().unwrap();
    main + rest
}

fn transform_srt_to_json(reader: BufReader<File>) -> Vec<VideoNote> {
    let mut video_notes: Vec<VideoNote> = vec![];
    let mut state = VideoNote {
        id: 0,
        payload: VideoNotePayload {
            r#type: "default".to_string(),
            content: "".to_string(),
            references: None,
        },
        start: 0.0,
        end: 0.0,
    };

    for line in reader
        .lines()
        .map(|line| line.unwrap())
        .filter(|one_line| one_line.len() > 0)
    {
        if let Ok(id) = line.parse::<i32>() {
            video_notes.push(state.clone());
            state.id = id;
            state.payload.content = "".to_string();
            state.start = 0.0;
            state.end = 0.0;
            continue;
        }
        if let Some(_) = line.find("-->") {
            let valami = line.to_string();
            let times: Vec<&str> = valami.split(" --> ").collect();
            state.start = parse_time_string_to_float(&times[0]);
            state.end = parse_time_string_to_float(&times[1]);
            continue;
        }
        if state.payload.content != "" {
            state.payload.content.push_str(&String::from("\n"));
        }
        state.payload.content.push_str(&line.to_string());
    }
    video_notes
}

#[tauri::command]
async fn import_srt_file<R: Runtime>(_app: AppHandle<R>, file_name: String) -> String {
    let absolute_path = format!(
        "{}/{}",
        download_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap(),
        file_name
    );

    let file = File::open(absolute_path).unwrap();
    let reader = BufReader::new(file);
    let video_notes: Vec<VideoNote> = transform_srt_to_json(reader);
    video_notes.to_html(vec![])
}

#[tauri::command]
fn open_video_notes<R: Runtime>(_app: AppHandle<R>, state: State<'_, PluginState>) -> String {
    let video_notes: &Vec<VideoNote> = &*state.loaded_notes.lock().unwrap();
    let chapters = &*state.chapters.lock().unwrap();
    video_notes.to_html(chapters.to_vec())
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
async fn seek_content<R: Runtime>(app: AppHandle<R>, time: f32) {
    let provider = app.get_window("video-player").unwrap();
    println!("TIME: {:?}", time);
    provider
        .eval(format!("window.__videoPlayer.time = {}", time).as_str())
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
    let video_notes = data.unwrap();
    let resp = reqwest::get("http://127.0.0.1:3000/chapters")
        .await
        .unwrap();
    let data = resp.json::<Vec<VideoChapter>>().await;
    let chapters = data.unwrap();
    *state.loaded_notes.lock().unwrap() = video_notes.clone();
    *state.chapters.lock().unwrap() = chapters.clone();

    for note in video_notes {
        let end_note = VideoNoteEnd {
            action_time: note.end,
            payload: note.payload,
            id: note.id,
        };
        state.end_notes.lock().unwrap().push(end_note);
    }

    println!("LOADED!");
    app.get_window("main")
        .unwrap()
        .emit("videonotes://notes-loaded", "")
        .unwrap();
    app.get_window("main")
        .unwrap()
        .emit("videonotes://chapters-loaded", chapters)
        .unwrap();
    Ok(())
}

fn convert_editor_video_note_to_video_note(item: Value)-> VideoNote {
    let mut video_note_content = String::from("");

    for fragment in item["content"].as_array().unwrap() {
        let mut text = String::from("");
        
        if fragment["type"].as_str() == Some("text") {
            text = fragment["text"].as_str().unwrap().to_string();

            if fragment["marks"].as_array() != None {
                text = format!("<strong>{}</strong>", &fragment["text"].as_str().unwrap());
            }
        }

        match fragment["type"].as_str().unwrap() {
            "text" => video_note_content.push_str(&text),
            "hardBreak" => video_note_content.push_str("\n"),
            _ => video_note_content.push_str(""),
        }
    }

    return VideoNote {
        id: item["attrs"]["id"].as_i64().unwrap() as i32,
        start: item["attrs"]["start"].as_f64().unwrap() as f32,
        end: item["attrs"]["end"].as_f64().unwrap() as f32,
        payload: VideoNotePayload {
            content: video_note_content.to_string(),
            r#type: item["attrs"]["type"].as_str().unwrap().to_string(),
            references: convert_references_to_video_note_references(item["attrs"]["references"].as_object().unwrap().clone()),
        }                            
    }
}

fn convert_references_to_video_note_references(references: Map<std::string::String, Value>) -> Option<Vec<VideoNoteReference>>{
    if references.is_empty() {
        return None;
    }

    let mut video_note_references = Vec::<VideoNoteReference>::with_capacity(100);

    for (id, phrases) in references {
        for phrase in phrases.as_array().unwrap() {
            video_note_references.push(VideoNoteReference { id: id.parse().unwrap(), phrase: phrase.clone().as_str().unwrap().to_string() });   
        }
    }

    Some(video_note_references)
}

#[tauri::command]
async fn save_notes<R: Runtime>(
    _app: AppHandle<R>,
    editor_json: EditorContent,
) -> Result<String, ()> {
    let mut chapters = Vec::<VideoChapter>::with_capacity(100);
    let mut notes = Vec::<VideoNote>::with_capacity(100);
    let content: Vec<Value> = editor_json.content;

    for item in content {
        match item["type"].as_str() {
            Some("chapter") => {
                chapters.push(VideoChapter {
                    id: item["attrs"]["id"].as_i64().unwrap() as i32,
                    title: item["attrs"]["title"].as_str().unwrap().to_string(),
                    start: item["attrs"]["start"].as_f64().unwrap() as f32,
                    end: item["attrs"]["end"].as_f64().unwrap() as f32,                            
                });
                let chapter_content = item["content"].clone();
                if let Value::Array(video_notes) = chapter_content  {
                    for item in video_notes {
                        notes.push(convert_editor_video_note_to_video_note(item))
                    }
                }
            },
            Some("videoNote") => notes.push(convert_editor_video_note_to_video_note(item)),
            _ => print!("Else"),
        }
    }
    
    Ok("Result".to_string())
}

fn round(number: f32) -> f32 {
    format!("{:.1}", number).parse::<f32>().unwrap()
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("videonote")
        .setup(|app: &AppHandle<R>| {
            app.manage(PluginState {
                loaded_notes: Mutex::new(Vec::<VideoNote>::with_capacity(1000)),
                end_notes: Mutex::new(Vec::<VideoNoteEnd>::with_capacity(1000)),
                chapters: Mutex::new(Vec::<VideoChapter>::with_capacity(50)),
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
                let end_notes: &Vec<VideoNoteEnd> = &state.end_notes.lock().unwrap();
                let offset = 0.3;

                if video_event_name == "timeupdate" {
                    // find start action
                    let mut index = 0;
                    let mut video_events: Vec<VideoNote> = Vec::<VideoNote>::with_capacity(5);
                    loop {
                        if index + 1 == video_notes.len() {
                            break;
                        }
                        let video_note: &VideoNote = &video_notes[index];
                        index = index + 1;
                        let start_time_rounded: f32 = round(video_note.start);
                        let current_time_rounded: f32 = round(video_event.time);
                        let lower_bound_rounded: f32 = round(current_time_rounded - offset);
                        // time - 0.20 <= start <= time
                        if lower_bound_rounded <= start_time_rounded
                            && start_time_rounded <= current_time_rounded
                        {
                            video_events
                                .push(video_notes[if index > 0 { index - 1 } else { 0 }].clone());
                        }
                    }

                    if video_events.len() != 0 {
                        main_window
                            .unwrap()
                            .emit("videonotes://start-notes", video_events)
                            .unwrap();
                    }

                    // find end action
                    let mut index = 0;
                    let mut video_events: Vec<VideoNote> = Vec::<VideoNote>::with_capacity(6);
                    let main_window_copy = app_copy.get_window("main");
                    loop {
                        if index + 1 == end_notes.len() {
                            break;
                        }
                        let video_note: &VideoNoteEnd = &end_notes[index];
                        index = index + 1;
                        let start_time_rounded: f32 = round(video_note.action_time);
                        let current_time_rounded: f32 = round(video_event.time);
                        let lower_bound_rounded: f32 = round(current_time_rounded - offset);
                        // time - 0.20 <= start <= time....
                        if lower_bound_rounded <= start_time_rounded
                            && start_time_rounded <= current_time_rounded
                        {
                            video_events
                                .push(video_notes[if index > 0 { index - 1 } else { 0 }].clone());
                        }
                    }
                    if video_events.len() != 0 {
                        main_window_copy
                            .unwrap()
                            .emit("videonotes://end-notes", video_events)
                            .unwrap();
                    }
                }
                let main_window = app_copy.get_window("main");
                main_window
                    .unwrap()
                    .emit("videonotes://video-player-event", video_event)
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
            import_srt_file,
            open_video_notes,
            save_notes,
        ])
        .build()
}
