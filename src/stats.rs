use serde::Deserialize;
use std::{collections::HashMap, error::Error, fs::File};

#[derive(Debug, Deserialize)]
pub struct ProStats {
    position: String,
    playername: String,
    teamname: String,
    kills: Option<f64>,
    deaths: Option<f64>,
    assists: Option<f64>,
    #[serde(rename = "total cs")]
    total_cs: Option<f64>,
    league: String,
    #[serde(skip)]
    points: f64,
}

//Reading from CSV
pub fn store_player() -> Result<Vec<ProStats>, Box<dyn Error>> {
    let file = File::open("test3.csv")?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let mut pro_map: HashMap<String, ProStats> = HashMap::new();

    for result in rdr.deserialize() {
        let mut record: ProStats = result?;
        if record.league == "LCK" && !record.playername.is_empty() {
            calculate_pts(&mut record);
            pro_map
                .entry(record.playername.clone())
                .and_modify(|e| {
                    e.points += record.points;
                    e.kills = Some(e.kills.unwrap_or(0.0) + record.kills.unwrap_or(0.0));
                    e.deaths = Some(e.deaths.unwrap_or(0.0) + record.deaths.unwrap_or(0.0));
                    e.assists = Some(e.assists.unwrap_or(0.0) + record.assists.unwrap_or(0.0));
                    e.total_cs = Some(e.total_cs.unwrap_or(0.0) + record.total_cs.unwrap_or(0.0));
                    e.points = (e.points * 100.0).round() / 100.0; // Round to 2 decimal places
                })
                .or_insert(record);
        }
    }

    let mut pro_list: Vec<ProStats> = pro_map.into_values().collect();

    pro_list.sort_by(|a, b| {
        a.teamname
            .cmp(&b.teamname)
            .then_with(|| role_order(&a.position).cmp(&role_order(&b.position)))
    });

    Ok(pro_list)
}

fn role_order(role: &str) -> usize {
    match role {
        "top" => 1,
        "jng" => 2,
        "mid" => 3,
        "bot" => 4,
        "sup" => 5,
        _ => 6,
    }
}

fn calculate_pts(player: &mut ProStats) {
    let kills = player.kills.unwrap_or(0.0);
    let deaths = player.deaths.unwrap_or(0.0);
    let assists = player.assists.unwrap_or(0.0);
    let total_cs = player.total_cs.unwrap_or(0.0);

    if player.position == "top" || player.position == "jng" {
        player.points = (kills * 0.4) + (assists * 0.2) + (total_cs * 0.002) - (deaths * 0.15);
    } else if player.position == "mid" || player.position == "bot" {
        player.points = (kills * 0.4) + (assists * 0.15) + (total_cs * 0.0015) - (deaths * 0.2);
    } else {
        player.points = (kills * 0.3) + (assists * 0.25) + (total_cs * 0.003) - (deaths * 0.1);
    }

    player.points = (player.points * 100.0).round() / 100.0;
}
