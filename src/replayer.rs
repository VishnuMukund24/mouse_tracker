use crate::MouseEvent;
use std::fs::File;
use std::io::{BufReader};
use csv::ReaderBuilder;

#[derive(Debug)]
pub struct Replayer {
    pub events: Vec<MouseEvent>,
}

impl Replayer {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
        }
    }

    pub fn load_from_json(&mut self, filename: &str) {
        let file = File::open(filename).expect("Could not open JSON file");
        let reader = BufReader::new(file);

        self.events = serde_json::from_reader(reader).expect("Could not parse JSON file");
        println!("Loaded {} events from JSON", self.events.len());
    }

    pub fn load_from_csv(&mut self, filename: &str) {
        let file = File::open(filename).expect("Could not open CSV file");
        let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

        let mut events = Vec::new();
        for result in rdr.records() {
            let record = result.expect("Could not read CSV record");
            let x: i32 = record[0].parse().expect("Invalid x");
            let y: i32 = record[1].parse().expect("Invalid y");
            let button = if &record[2] == "None" {None} else { Some(record[2].to_string()) };
            let time:f64 = record[3].parse().expect("Invalid time");

            events.push(MouseEvent { x, y, button, time });
        }
        self.events = events;
        println!("Loaded {} events from CSV", self.events.len());
    }

    pub fn replay(&self) {
        for event in &self.events {
            println!("{:?}", event);
        }
    }
}
