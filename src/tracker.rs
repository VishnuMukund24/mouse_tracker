use crate::MouseEvent;
use std::fs::File;
use std::io::{Write, BufWriter};

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