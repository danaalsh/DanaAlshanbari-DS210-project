// mod for data analysis 
// shows correlation between average study hours vs exam scores on a line plot using plotters

use crate::data::Student;
use plotters::prelude::*;

// function that creates line plot showing average exam score by binned study hours
// saves graph as "hours_vs_scores.png"
pub fn plot_hours_vs_score(students: &[Student]) -> Result<(), Box<dyn std::error::Error>> {
    let mut bins = vec![vec![]; 10];

    for s in students {
        let hour_bin = (s.study_hours_per_week / 10.0).floor() as usize;
        if hour_bin < bins.len() {
            bins[hour_bin].push(s.score);
        }
    }

    // computes the averages of the bins
    let mut data: Vec<(u32, f32)> = bins.iter().enumerate()
        .filter(|(_, score)| !score.is_empty())
        .map(|(i, score)| {
            let avg = score.iter().sum::<f32>() / score.len() as f32;
            ((i as u32) * 10, avg)
        })
        .collect();

    // plots graph using Plotters
    let root = BitMapBackend::new("hours_vs_scores.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_x = 50u32;
    let max_y = data.iter().map(|&(_, y)| y).fold(0.0, f32::max).ceil() + 5.0;

    let mut graph = ChartBuilder::on(&root)
        .caption("Average Exam Score vs Study Hours", ("sans-serif", 24))
        .margin(40)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(0u32..max_x, 0f32..max_y)?;

    graph.configure_mesh()
        .x_desc("Study Hours (binned)")
        .y_desc("Average Exam Score")
        .draw()?;

    graph.draw_series(LineSeries::new(data.clone(), &BLUE))?
        .label("Avg Exam Score")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    graph.draw_series(
        data.iter().map(|(x, y)| Circle::new((*x, *y), 4, RED.filled()))
    )?;

    graph.configure_series_labels()
        .background_style(&WHITE)
        .border_style(&BLACK)
        .draw()?;

    println!("Chart saved to hours_vs_scores.png");
    Ok(())

}

