use arboard::Clipboard;
use std::{fs, thread};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use chrono::Utc;
use crate::clip_model::ClipEntry;

const HISTORY_FILE: &str = "/tmp/cliphistory/clipboard_history.json";
const HISTORY_SIZE: u8 = 150;
const SAVE_INTERVAL: Duration = Duration::from_secs(30); // Save history every 30 seconds

// Shared state
lazy_static::lazy_static! {
    static ref HISTORY: Arc<Mutex<Vec<ClipEntry>>> = Arc::new(Mutex::new(load_history()));
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

pub fn listen_clipboard() {
    let mut clipboard = Clipboard::new().expect("Failed to initialize clipboard");
    let mut last_clip: Option<String> = clipboard.get_text().ok();

    // Create the path if it doesn't exist
    fs::create_dir_all("/tmp/cliphistory").expect("Failed to create /tmp/cliphistory directory");

    // Start the background task for saving history periodically
    let history_clone = Arc::clone(&HISTORY);
    thread::spawn(move || {
        save_history_periodically(history_clone);
    });

    loop {
        if let Ok(text) = clipboard.get_text() {
            if Some(&text) != last_clip.as_ref() {
                println!("New clipboard entry detected");

                // Lock history and add the new clipboard entry
                let mut history_guard = HISTORY.lock().expect("Failed to lock history");

                // Ensure history size does not exceed the limit
                if history_guard.len() >= HISTORY_SIZE as usize {
                    clear_oldest_value(&mut history_guard);
                }

                let trimmed_text = text.trim().to_string();
                history_guard.push(ClipEntry {
                    timestamp: Utc::now().to_rfc3339(),
                    content: trimmed_text.clone(),
                });

                // Update the last clipboard content
                last_clip = Some(trimmed_text);
            }
        }

        // Sleep for a short time to prevent constant polling (reduce CPU usage)
        thread::sleep(Duration::from_millis(200));
    }
}

fn save_history_periodically(history: Arc<Mutex<Vec<ClipEntry>>>) {
    let mut last_saved = Instant::now();

    loop {
        if last_saved.elapsed() >= SAVE_INTERVAL {
            let history_guard = history.lock().expect("Failed to lock history");
            save_history(&history_guard);
            last_saved = Instant::now(); // Reset timer after saving
        }

        // Sleep for a short interval to reduce CPU usage in the saving thread
        thread::sleep(Duration::from_millis(100));
    }
}

fn save_history(history: &Vec<ClipEntry>) {
    let json = serde_json::to_string_pretty(history).expect("Failed to serialize history");
    fs::write(HISTORY_FILE, json).expect("Failed to save history");
}

fn load_history() -> Vec<ClipEntry> {
    if let Ok(data) = fs::read_to_string(HISTORY_FILE) {
        serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}

fn clear_oldest_value(history: &mut Vec<ClipEntry>) {
    if !history.is_empty() {
        history.remove(0); // Remove the oldest (first) entry
    }
}
