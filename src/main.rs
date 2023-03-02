extern crate csv;

use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    // Path to the CSV file
    let path = Path::new("sample.csv");

    // Create a new CSV reader
    let mut rdr = csv::Reader::from_path(path).map_err(|e| format!("Error creating CSV reader: {}", e))?;

    // Iterate over each record in the CSV file
    for result in rdr.records() {
        let record = result.map_err(|e| format!("Error reading record: {}", e))?;

        // Validate the record
        if record.len() < 3 {
            eprintln!("Invalid record: {:?}", record);
            continue;
        }

        let name = &record[0];
        let age = match record[1].parse::<i32>() {
            Ok(n) if n >= 0 => n,
            _ => {
                eprintln!("Invalid age: {:?}", record);
                continue;
            }
        };
        let email = &record[2];

        // Do something with the validated data
        println!("Name: {}, Age: {}, Email: {}", name, age, email);
    }

    Ok(())
}
