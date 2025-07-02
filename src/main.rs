mod replayer;

use rdev::{listen, Event, EventType, Button, Key};
use std::{process::exit, sync::{Arc, Mutex}};
use std::time::Instant;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Write, BufWriter};

use crate::replayer::Replayer;  // <--- Replayer struct

#[derive(Debug, Serialize, Deserialize)]
pub struct MouseEvent {
    pub x: i32,
    pub y: i32,
    pub button: Option<String>,
    pub time: f64,
}

// Tracker struct
pub struct Tracker {
    pub events: Vec<MouseEvent>,
}

impl Tracker {
    pub fn new() -> Self {
        Tracker {
            events: Vec::new(),
        }
    }

    pub fn add_event(&mut self, x: i32, y: i32, button: Option<String>, time: f64) {
        self.events.push(MouseEvent { x, y, button, time });
    }

    pub fn save_as_csv(&self, filename: &str) {
        let file = File::create(filename).expect("Could not create CSV file");
        let mut writer = BufWriter::new(file);

        writeln!(writer, "x,y,button,time").expect("Could not write header");

        for event in &self.events {
            let button = event.button.clone().unwrap_or_else(|| "None".to_string());
            writeln!(writer, "{},{},{},{}", event.x, event.y, button, event.time).expect("Could not write row");
        }

        println!("Events saved to {}", filename);
    }

    pub fn save_as_json(&self, filename: &str) {
        let file = File::create(filename).expect("Could not create JSON file");
        serde_json::to_writer_pretty(file, &self.events).expect("Could not write JSON");

        println!("Events saved to {}", filename);
    }
}

fn main() {
    let start = Instant::now();
    let tracker = Arc::new(Mutex::new(Tracker::new()));
    let position = Arc::new(Mutex::new((0, 0)));

    let tracking_enabled = Arc::new(Mutex::new(false)); // Tracking flag
    
    let tracker_clone = Arc::clone(&tracker);
    let position_clone = Arc::clone(&position);
    let tracking_enabled_clone = Arc::clone(&tracking_enabled);

    println!("Press 'K' to start tracking.");
    println!("Press 'Space' to replay events.");
    println!("Press ESC to save and exit.");

    let callback = move |event: Event| {
        let mut tracker = tracker_clone.lock().unwrap();
        let mut pos = position_clone.lock().unwrap();
        let mut tracking = tracking_enabled_clone.lock().unwrap();

        let elapsed = start.elapsed();
        let seconds = elapsed.as_secs_f64();

        match event.event_type {
            EventType::MouseMove { x, y } => {
                if *tracking {
                    *pos = (x as i32, y as i32);
                    tracker.add_event(x as i32, y as i32, None, seconds);
                }    
            }
            EventType::ButtonPress(button) => {
                if *tracking {
                    let label = match button {
                        Button::Left => "Left",
                        Button::Right => "Right",
                        Button::Middle => "Middle",
                        _ => "Other",
                    };
                    let (x, y) = *pos;
                    tracker.add_event(x, y, Some(label.to_string()), seconds);
                }
            }
            EventType::KeyPress(key) => {
                match key {
                    Key::KeyK => {
                        if *tracking {
                            // Was tracking — now stopping.
                            println!("Stopping tracking and saving...");

                            tracker.save_as_csv("logs/mouse_events.csv");
                            // tracker.save_as_json("logs/mouse_events.json");

                            // Reset tracker for next session if you want:
                            *tracker = Tracker::new();

                            *tracking = false;
                            println!("Tracking stopped. Press K to start again, or Space to replay.");
                        } else {
                            *tracking = true;
                            println!("Tracking started.");
                        }
                    }
                    Key::Space => {
                        println!("Replaying saved events...");

                        let mut replayer = Replayer::new();

                        // Load from JSON or CSV — your choice
                        // replayer.load_from_json("logs/mouse_events.json");
                        // OR:
                        replayer.load_from_csv("logs/mouse_events.csv");

                        replayer.replay();
                        println!("Replaying stopped. Press K to start again, or Space to replay.");
                    }
                    Key::Escape => {
                        println!("Escape pressed. Exiting...");

                        // Optional final save if you want to catch any last tracking session:
                        if *tracking {
                            tracker.save_as_csv("logs/mouse_events.csv");
                            // tracker.save_as_json("logs/mouse_events.json");
                            println!("Final session saved before exit.");
                        }

                        exit(0);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    };

    if let Err(err) = listen(callback) {
        eprintln!("Error: {:?}", err);
    }
}
