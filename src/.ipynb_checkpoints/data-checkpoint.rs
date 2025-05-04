// mod for loading the csv file

use std::fs::File;
use std::io::BufReader;
use csv::Reader;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Student {
    #[serde(rename = "Study_Hours_per_Week")]
    pub study_hours_per_week: f32,

    #[serde(rename = "Final_Grade")]
    pub grade: String,
}


pub fn read_data(path: &str) -> Vec<Student> {
    let file = File::open(path).expect("Could not open file");
    let mut reader = Reader::from_reader(BufReader::new(file));

    let mut students = Vec::new();

    for result in reader.deserialize::<Student>() {
        match result {
            Ok(student) => students.push(student),
            Err(err) => eprintln!("Skipping invalid row: {}", err),
        }
    }

    students
}