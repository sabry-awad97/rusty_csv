extern crate csv;
extern crate plotters;

use plotters::prelude::*;
// use rand::Rng;
use std::error::Error;
// use std::fs::File;
// use std::io::Write;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    // Path to the CSV file
    let path = Path::new("sample.csv");

    // Create a new CSV reader
    let mut rdr = csv::Reader::from_path(path)?;

    // Count the number of records for each age
    let mut age_counts = std::collections::HashMap::new();
    for result in rdr.records() {
        let record = result?;
        let age = record[1].parse::<i32>().unwrap_or(-1);
        if age >= 0 {
            *age_counts.entry(age).or_insert(0) += 1;
        }
    }

    // Create a bar chart of the age counts
    let root = BitMapBackend::new("age_counts.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Age Counts", ("sans-serif", 18))
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..150, 0..10)?;

    chart
        .configure_mesh()
        .x_desc("Age")
        .y_desc("Count")
        .draw()?;

    let data = age_counts.into_iter().collect::<Vec<_>>();
    chart.draw_series(
        data.iter()
            .map(|(x, y)| (*x, *y))
            .map(|(x, y)| Rectangle::new([(x, 0), ((x + 1), y)], BLUE.filled())),
    )?;

    Ok(())
}

// fn generate_sample() -> Result<(), Box<dyn Error>> {
//     let path = Path::new("large_sample.csv");
//     let mut file = File::create(path)?;

//     let mut rng = rand::thread_rng();

//     // Write header row
//     writeln!(file, "id,age")?;

//     // Generate 100,000 records with random ages between 0 and 99
//     for i in 1..=100_000 {
//         let age = rng.gen_range(18..118);
//         writeln!(file, "{},{}", i, age)?;
//     }

//     Ok(())
// }
