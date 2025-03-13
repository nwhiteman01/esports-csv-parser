mod stats;

use std::process;

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
    match stats::store_player() {
        Ok(players) => {
            for player in players {
                println!("{:?}", player);
            }
        }
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    }
}
