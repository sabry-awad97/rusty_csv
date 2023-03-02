extern crate csv;

use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    // Path to the CSV file
    let path = Path::new("sample.csv");

    // Create a new CSV reader
    let mut rdr = csv::Reader::from_path(path)?;

    // Iterate over each record in the CSV file
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}
