use crate::MouseEvent;
// use serde::{Deserialize};
use std::fs::File;
use std::io::{BufReader};
use std::{thread, time::{Duration, Instant}};
use rdev::{simulate, EventType, Button};


#[derive(Debug)]
pub struct Replayer {
    pub events: Vec<MouseEvent>,
}

impl Replayer {
    pub fn new_from_json(path: &str) -> Self {
        let file = File::open(path).expect("Failed to open file");
        let reader = BufReader::new(file);
        let events: Vec<MouseEvent> = serde_json::from_reader(reader).expect("Failed to parse JSON");
        Self { events }
    }

    pub fn new_from_csv(path: &str) -> Self {
        let mut rdr = csv::Reader::from_path(path).expect("Failed to open file");
        let mut events = Vec::new();
        for result in rdr.deserialize() {
            let record: MouseEvent = result.expect("Failed to parse CSV");
            events.push(record);
        }
        Self { events }
    }

    pub fn new_from_bin(path: &str) -> Self {
        let file = File::open(path).expect("Failed to open binary file");
        let reader = BufReader::new(file);
        let events: Vec<MouseEvent> = bincode::deserialize_from(reader).expect("Failed to deserialize");
        Self { events }
    }

    pub fn replay(&self) {
        if self.events.is_empty() {
            println!("No events to replay!");
            return;
        }
        let first_ts = self.events.first().map(|e| e.time).unwrap_or(0.0);

        // 1️⃣ Mark the start of replay
        let start = Instant::now();
        println!("Starting {:?}", start);
        for event in &self.events {
            // Compute normalized elapsed time since the very first event:
            let normalized = event.time - first_ts;
            // 2️⃣ Compute the absolute target time for this event
            let target = start + Duration::from_secs_f64(normalized);

            // 3️⃣ Sleep until that target (or skip if we're already late)
            let now = Instant::now();
            if now < target {
                thread::sleep(target - now);
            }

            // 4️⃣ Simulate the mouse move
            if let Err(e) = simulate(&EventType::MouseMove {
                x: event.x as f64,
                y: event.y as f64,
            }) {
                eprintln!("Move failed: {:?}", e);
            }

            // 5️⃣ If there’s a click, simulate press & release with a tiny gap
            if let Some(btn_str) = &event.button {
                let btn = match btn_str.as_str() {
                    "Left"   => Button::Left,
                    "Right"  => Button::Right,
                    "Middle" => Button::Middle,
                    _        => continue,
                };
                if let Err(e) = simulate(&EventType::ButtonPress(btn)) {
                    eprintln!("Press failed: {:?}", e);
                }
                // tiny gap to ensure OS registers the click
                thread::sleep(Duration::from_millis(5));
                if let Err(e) = simulate(&EventType::ButtonRelease(btn)) {
                    eprintln!("Release failed: {:?}", e);
                }
            }
        }
    }
}
