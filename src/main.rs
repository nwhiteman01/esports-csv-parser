use serde::Deserialize;
use std::{error::Error, fs::File, process};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ProStats {
    playername: String,
    teamname: String,
    kills: Option<u64>,
    deaths: Option<u64>,
    assists: Option<u64>,
    #[serde(rename = "total cs")]
    total_cs: Option<u64>,
    league: String,
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

fn main() {
    if let Err(err) = store_player() {
        println!("{}", err);
        process::exit(1);
    }
}

//Reading from CSV
fn store_player() -> Result<(), Box<dyn Error>> {
    // Create a CSV parser that reads data from file.
    let file = File::open("test3.csv")?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);
    // Loop over each record.
    for result in rdr.deserialize() {
        let record: ProStats = result?;
        if record.league == "LCK" && !record.playername.is_empty() {
            println!("{:?}", record);
        }
    }
    Ok(())
}
