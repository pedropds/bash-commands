use arboard::Clipboard;
use std::{fs, thread};
use std::sync::{Arc, Mutex};
use chrono::Utc;
use crate::clip_model::ClipEntry;

const HISTORY_FILE: &str = "clipboard_history.json";
const HISTORY_SIZE: u8 = 150;

pub fn listen_clipboard() {
    let mut clipboard = Clipboard::new().expect("Failed to initialize clipboard");
    let mut last_clip: Option<String>;

    if let Ok(text) = clipboard.get_text() {
        last_clip = Some(text);
    } else {
        last_clip = None;
    }

    let history = Arc::new(Mutex::new(load_history()));

    loop {
        if let Ok(text) = clipboard.get_text() {
            if Some(&text) != last_clip.as_ref() {
                println!("New clipboard entry detected");

                // Lock the history and add the new clipboard entry
                let mut history_guard = history.lock().expect("Failed to lock history");

                // Ensure the history doesn't exceed the size limit
                if history_guard.len() >= HISTORY_SIZE as usize {
                    clear_oldest_value(&mut history_guard);
                }

                let trimmed_text = text.trim().to_string();

                history_guard.push(ClipEntry {
                    timestamp: Utc::now().to_rfc3339(),
                    content: trimmed_text.clone(),
                });

                // Call the async save function with the Arc<Mutex>
                save_history_async(Arc::clone(&history));

                last_clip = Some(trimmed_text);
            }
        }
    }
}

pub fn list_history() {
    let history = load_history();
    for entry in history {
        println!("{}: {}", entry.timestamp, entry.content);
    }
}

pub fn clear_history() {
    fs::write(HISTORY_FILE, "[]").expect("Failed to clear history");
    println!("Clipboard history cleared.");
}

fn load_history() -> Vec<ClipEntry> {
    if let Ok(data) = fs::read_to_string(HISTORY_FILE) {
        serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}

fn save_history_async(history_guard: Arc<Mutex<Vec<ClipEntry>>>) {
    thread::spawn(move || {
        let json = serde_json::to_string_pretty(&*history_guard).expect("Failed to serialize history");
        fs::write(HISTORY_FILE, json).expect("Failed to save history");

        println!("History saved in background.");
    });
}

fn clear_oldest_value(history: &mut Vec<ClipEntry>) {
    if !history.is_empty() {
        history.remove(0); // Remove the oldest (first) entry
    }
}