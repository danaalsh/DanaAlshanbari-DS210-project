// mod for loading the csv file

use std::fs::File;
use std::io::BufReader;
use csv::Reader;
use serde::Deserialize;

// Struct that represents a student's data: weekly study hours and exam score. Used for correlation analysis
#[derive(Debug, Deserialize)]
pub struct Student {
    #[serde(rename = "Study_Hours_per_Week")]
    pub study_hours_per_week: f32,

    #[serde(rename = "Exam_Score (%)")]
    pub score: f32,
}

// function that loads student's data from the csv file
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

// One Test
#[cfg(test)]
mod tests {
    use super::*;

    // Confirms that read_data correctly loads and parses rows from the test dataset
    #[test]
    fn test_load_data() {
        let students = read_data("test_data.csv");

        // Checking if it loaded all 9 rows
        assert_eq!(students.len(), 9);

        // Check one row/student
        assert_eq!(students[3].study_hours_per_week, 13.0);
        assert_eq!(students[3].score, 70.0);
    }
}