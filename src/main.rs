mod data;
mod analysis;

use data::{read_data, Student};
use crate::analysis::plot_hours_vs_score;

fn main() {

    let students = read_data("student_performance_large_dataset.csv");

    if let Err(err) = plot_hours_vs_score(&students) {
        eprintln!("Error generating plot: {}", err);
    }
    
}
