//mod for data analysis (analyzing average study hours and comparing it to average final grade )

use crate::data::Student;

// function that computes average study hours
pub fn avg_hours(students: &[Student]) -> f32 {
    if students.is_empty() {
        return 0.0;
    }

    let total: f32 = students.iter().map(|s| s.study_hours_per_week).sum();
    total / students.len() as f32
}

