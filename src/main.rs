use rdev::{listen, Event, EventType, Button, Key};
use std::{process::exit, sync::{Arc, Mutex}};


#[derive(Debug)]
pub struct MouseEvent {
    pub x: i32,
    pub y: i32,
    pub button: Option<String>,
} // Struct for the values of mouse inputs

pub struct Tracker {
    pub events: Vec<MouseEvent>,
    // pub stop  : bool,
} // Struct to store the events of mouse in the form of vector referencing 'MouseEvent' struct

impl Tracker {
    pub fn new() -> Self {
        Tracker {
            events: Vec::new(), // initializes events field as an empty vector
            // stop: false,
        }
    } // Constructor function which doesn't take 'self', just creates a new instance of struct'
    // 'Self' is an alias for the type the impl block is for - this function returns a 'Tracker' instance

    pub fn add_event(&mut self, x: i32, y:i32, button: Option<String>) {
        self.events.push(MouseEvent { x, y, button });
    }

    pub fn print_events(&self) {
        for event in &self.events {
            println!("{:?}", event);
        }
    }
}  // Implementation Block allows to define associated functions and methods for the struct


fn main() {
    let tracker = Arc::new(Mutex::new(Tracker::new()));
    let position = Arc::new(Mutex::new((0,0)));

    let tracker_clone = Arc::clone(&tracker);
    let position_clone = Arc::clone(&position);

    println!("Tracking mouse... Press ESC to stop.");

    let callback = move |event: Event| {
        let mut tracker = tracker_clone.lock().unwrap();
        let mut pos = position_clone.lock().unwrap();

        match event.event_type {
            EventType::MouseMove { x, y } => {
                *pos = (x as i32, y as i32);
                tracker.add_event(x as i32, y as i32, None);
            }
            EventType::ButtonPress(button) => {
                let label = match button {
                    Button::Left   => "Left",
                    Button::Right  => "Right",
                    Button::Middle => "Middle",
                    _ => "Other",
                };
                let (x, y) = *pos;
                tracker.add_event(x, y, Some(label.to_string()));
            }
            EventType::KeyPress(key) => {
                if key == Key::Escape {
                    // tracker.stop = true;
                    println!("\nEscape pressed. Exiting and printing tracked events:\n");
                    tracker.print_events();
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
    // tracker.lock().unwrap().print_events();
}