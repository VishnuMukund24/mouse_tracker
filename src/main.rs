mod tracker;
mod replayer;


use rdev::{listen, Event, EventType, Button, Key};
use std::{process::exit, sync::{Arc, Mutex}};
use std::time::Instant;
use serde::{Serialize, Deserialize};

use crate::tracker::Tracker;    // <--- Tracker struct
use crate::replayer::Replayer;  // <--- Replayer struct

#[derive(Debug, Serialize, Deserialize)]
pub struct MouseEvent {
    pub x: i32,
    pub y: i32,
    pub button: Option<String>,
    pub time: f64,
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

                            // tracker.save_as_csv("logs/mouse_events.csv");
                            // tracker.save_as_json("logs/mouse_events.json");
                            tracker.save_as_bin("logs/mouse_events.bin");

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
                        if *tracking {
                            println!("Exiting tracking..");
                            // tracker.save_as_csv("logs/mouse_events.csv");
                            // tracker.save_as_json("logs/mouse_events.json");
                            tracker.save_as_bin("logs/mouse_events.bin");
                            // Reset tracker for next session if you want:
                            *tracker = Tracker::new();
                            *tracking = false;
                        }
                        println!("Replaying saved events...");    
                        // let replayer = Replayer::new_from_csv("logs/mouse_events.csv");
                        // let replayer = Replayer::new_from_json("logs/mouse_events.json");
                        let replayer = Replayer::new_from_bin("logs/mouse_events.bin");
                        // Load from JSON or CSV — your choice
                        replayer.replay();
                        println!("Replaying stopped. Press K to start again, or Space to replay.");
                    }
                    Key::Escape => {
                        println!("Escape pressed. Exiting...");

                        // Optional final save if you want to catch any last tracking session:
                        if *tracking {
                            // tracker.save_as_csv("logs/mouse_events.csv");
                            tracker.save_as_json("logs/mouse_events.json");
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
