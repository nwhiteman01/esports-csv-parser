use std::{process, fs::File, error::Error};
use serde::Deserialize;


#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ProStats {
    playername: String,
    teamname: String,
    kills: u64,
    deaths: u64,
    assists: u64,
    #[serde(rename = "total cs")]
    total_cs: u64,
}

/*
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    latitude: f64,
    longitude: f64,
    population: Option<u64>,
    city: String,
    state: String,
}
*/


fn main(){
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

//Reading from CSV
fn run() -> Result<(), Box<dyn Error>> {
    // Create a CSV parser that reads data from file.
    let file = File::open("test2.csv")?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);
    // Loop over each record.
    for result in rdr.deserialize() {
        let record: ProStats = result?;
        println!("{:?}", record);
    }
    Ok(())
}

