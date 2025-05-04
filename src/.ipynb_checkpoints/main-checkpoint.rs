mod data;
mod analysis;

use data::{read_data, Student};

fn main() {
    let students = read_data("student_performance_large_dataset.csv");

    println!("✅ Loaded {} students", students.len());

    // Preview first few entries
    for (i, student) in students.iter().take(5).enumerate() {
        println!(
            "{}. Grade: {}, Study Hours: {}",
            i + 1,
            student.grade,
            student.study_hours_per_week
        );
    }
}
