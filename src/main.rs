use rdev::{listen, Event, EventType, Button, Key};
use std::{process::exit, sync::{Arc, Mutex}};
use std::time::Instant;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Write, BufWriter};


#[derive(Debug, Serialize, Deserialize)]
pub struct MouseEvent {
    pub x: i32,
    pub y: i32,
    pub button: Option<String>,
    pub time: f64,
} // Struct for the values of mouse inputs

pub struct Tracker {
    pub events: Vec<MouseEvent>,
} // Struct to store the events of mouse in the form of vector referencing 'MouseEvent' struct

impl Tracker {
    pub fn new() -> Self {
        Tracker {
            events: Vec::new(), // initializes events field as an empty vector
        }
    } // Constructor function which doesn't take 'self', just creates a new instance of struct'
    // 'Self' is an alias for the type the impl block is for - this function returns a 'Tracker' instance

    pub fn add_event(&mut self, x: i32, y:i32, button: Option<String>, time:f64) {
        self.events.push(MouseEvent { x, y, button, time });
    }

    pub fn print_events(&self) {
        for event in &self.events {
            println!("{:?}", event);
        }
    }

    pub fn save_as_csv(&self, filename: &str) {
        let file = File::create(filename).expect("Could not create CSV file");
        let mut writer = BufWriter::new(file);

        // Write CSV header
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
}  // Implementation Block allows to define associated functions and methods for the struct


fn main() {
    let start = Instant::now(); // here Instant implements Copy, so you can just pass it 
    let tracker = Arc::new(Mutex::new(Tracker::new()));
    let position = Arc::new(Mutex::new((0,0)));

    let tracker_clone = Arc::clone(&tracker);
    let position_clone = Arc::clone(&position);

    println!("Tracking mouse... Press ESC to stop.");

    let callback = move |event: Event| {
        let mut tracker = tracker_clone.lock().unwrap();
        let mut pos = position_clone.lock().unwrap();

        let elapsed = start.elapsed();
        let seconds = elapsed.as_secs_f64();

        match event.event_type {
            EventType::MouseMove { x, y } => {
                *pos = (x as i32, y as i32);  // smart pointer with MutexGuard
                tracker.add_event(x as i32, y as i32, None, seconds);
            }
            EventType::ButtonPress(button) => {
                let label = match button {
                    Button::Left   => "Left",
                    Button::Right  => "Right",
                    Button::Middle => "Middle",
                    _ => "Other",
                };
                let (x, y) = *pos;
                tracker.add_event(x, y, Some(label.to_string()), seconds);
            }
            EventType::KeyPress(key) => {
                if key == Key::Escape {
                    println!("\nEscape pressed. Exiting and printing tracked events:\n");
                    // tracker.print_events();
                    tracker.save_as_csv("logs/mouse_events.csv");
                    exit(0); // Gracefully end the program
                }
            }
            _ => {}
        }
    };

    if let Err(err) = listen(callback) {
        eprintln!("Error: {:?}", err);
    }

    // After ESC pressed and callback ends (listen exits)
}