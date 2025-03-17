use serde::{Deserialize, Deserializer};
use std::{collections::HashMap, error::Error, fs::File};

#[derive(Debug, Deserialize)]
pub struct ProStats {
    position: String,
    playername: String,
    teamname: String,
    #[serde(default, deserialize_with = "default_empty_string_to_f64")]
    kills: f64,
    #[serde(default, deserialize_with = "default_empty_string_to_f64")]
    deaths: f64,
    #[serde(default, deserialize_with = "default_empty_string_to_f64")]
    assists: f64,
    #[serde(rename = "total cs", default, deserialize_with = "default_empty_string_to_f64")]
    total_cs: f64,
    league: String,
    #[serde(default)]
    weeklypoints: f64,
    #[serde(default)]
    totalpoints: f64,
    #[serde(default)]
    result: u32,
}

#[derive(Debug, Default)]
pub struct Team {
    teamname: String,
    wins: u32, // Tracks the number of wins for the team
}

impl Team {
    /// Create a new team with a given name and zero initial wins
    pub fn new(teamname: String) -> Self {
        Self { teamname, wins: 0 }
    }

    pub fn teamname(&self) -> &str {
        &self.teamname
    }

    pub fn print_teams(teams: &[Team]) {
        println!("{:<20}{}", "Team Name", "Wins");
        for team in teams {
            println!("{:<20}{}", team.teamname, team.wins);
        }
    }
}


// Getter implementations are available within the impl block if needed.
impl ProStats {
    // Public getter for playername.
    pub fn playername(&self) -> &str {
        &self.playername
    }

    // Public getter for weeklypoints.
    pub fn weeklypoints(&self) -> f64 {
        self.weeklypoints
    }

    pub fn position(&self) -> &str {
        &self.position
    }

    /// Calculates and updates the weeklypoints based on the player's stats.
    pub fn calculate_points(&mut self) {
        let kills = self.kills;
        let deaths = self.deaths;
        let assists = self.assists;
        let total_cs = self.total_cs;

        if self.position == "top" || self.position == "jng" {
            self.weeklypoints = (kills * 0.4) + (assists * 0.2) + (total_cs * 0.002) - (deaths * 0.15);
        } else if self.position == "mid" || self.position == "bot" {
            self.weeklypoints = (kills * 0.4) + (assists * 0.15) + (total_cs * 0.0015) - (deaths * 0.2);
        } else {
            self.weeklypoints = (kills * 0.3) + (assists * 0.25) + (total_cs * 0.003) - (deaths * 0.1);
        }

        self.weeklypoints = (self.weeklypoints * 100.0).round() / 100.0;
    }

    /// Prints all players in the given slice.
    /// The players are grouped by team with a single header printed at the top.
    pub fn print_all(players: &[ProStats]) {
        Self::print_header();
        let mut current_team = String::new();
        for player in players {
            if player.teamname != current_team {
                if !current_team.is_empty() {
                    println!(); // blank line between teams
                }
                current_team = player.teamname.clone();
                println!("{}", current_team);
            }
            Self::print_player(player);
        }
    }

    pub fn print_by_name(players: &[ProStats], name: &str) {
        // Convert target name to lowercase once.
        let target = name.to_lowercase();
        if let Some(player) = players.iter().find(|p| p.playername.to_lowercase() == target) {
            Self::print_header();
            Self::print_player(player);
        } else {
            println!("Player not found: {}", name);
        }
    }

    pub fn print_by_name_no_header(players: &[ProStats], name: &str) {
        let target = name.to_lowercase();
        if let Some(player) = players.iter().find(|p| p.playername.to_lowercase() == target) {
            // Use internal helper to print a player's row.
            // (Assumes that print_player is a private function within the impl or module.)
            ProStats::print_player(player);
        } else {
            println!("Player not found: {}", name);
        }
    }
    
    pub fn print_by_team(players: &[ProStats], teamname: &str) {
        // Convert target team name to lowercase once.
        let target = teamname.to_lowercase();
        let filtered: Vec<&ProStats> = players
            .iter()
            .filter(|p| p.teamname.to_lowercase() == target)
            .collect();
        if filtered.is_empty() {
            println!("No players found for team: {}", teamname);
            return;
        }
        println!("{}", teamname);
        Self::print_header();
        for player in filtered {
            Self::print_player(player);
        }
    }
    

    /// Prints the top player (highest weeklypoints) for each role.
    pub fn print_top_role(players: &[ProStats]) {
        let roles = vec!["top", "jng", "mid", "bot", "sup"];
        println!("Top Players by Role:");
        Self::print_header();
        for role in roles {
            if let Some(top_player) = players
                .iter()
                .filter(|p| p.position == role)
                .max_by(|a, b| a.weeklypoints.partial_cmp(&b.weeklypoints).unwrap())
            {
                Self::print_player(top_player);
            }
        }
    }

    // Private helper to print the header.
    pub fn print_header() {
        println!(
            "{:<20}{:>10}{:>10}{:>10}{:>7}{:>10}{:>10}",
            "Name", "Kills", "Deaths", "Assists", "CS", "Weekly", "Total"
        );
    }

    // Private helper to print a single player's stats.
    pub fn print_player(player: &ProStats) {
        println!(
            "{:<20}{:>10}{:>10}{:>10}{:>7}{:>10.2}{:>10.2}",
            player.playername,
            player.kills as u32,
            player.deaths as u32,
            player.assists as u32,
            player.total_cs as u32,
            player.weeklypoints,
            player.totalpoints
        );
    }
}

// Helper function: Converts an empty string to 0.0 during deserialization.
fn default_empty_string_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<&str> = Option::deserialize(deserializer)?;
    Ok(s.unwrap_or("").parse::<f64>().unwrap_or(0.0))
}

pub fn store_player() -> Result<(Vec<ProStats>, Vec<Team>), Box<dyn Error>> {
    let file_players = File::open("pro_list.csv")?;
    let mut rdr_players = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file_players);

    let mut pro_map: HashMap<String, ProStats> = HashMap::new();
    let mut team_map: HashMap<String, Team> = HashMap::new();

    // Read records from pro_list.csv.
    for result in rdr_players.deserialize() {
        let record: ProStats = result?;
        pro_map.insert(record.playername.clone(), record);
    }

    let file_new_stats = File::open("test3.csv")?;
    let mut rdr_new_stats = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file_new_stats);

    for result in rdr_new_stats.deserialize() {
        let mut record: ProStats = result?;
        if !record.playername.is_empty() && record.league == "LCK" {
            record.calculate_points();
            pro_map
                .entry(record.playername.clone())
                .and_modify(|e| {
                    e.kills += record.kills;
                    e.deaths += record.deaths;
                    e.assists += record.assists;
                    e.total_cs += record.total_cs;
                    e.weeklypoints += record.weeklypoints;
                    e.totalpoints += record.weeklypoints;
                })
                .or_insert_with(|| {
                    record.totalpoints = record.weeklypoints;
                    record
                });
        } else if record.playername.is_empty() && record.league == "LCK" {
            team_map
                .entry(record.teamname.clone())
                .and_modify(|team| {
                    team.wins += record.result; // Increment the win count (or other result data)
                })
                .or_insert_with(|| {
                    let mut new_team = Team::new(record.teamname.clone());
                    new_team.wins = record.result; // Initialize with the result value
                    new_team
                });
        }
    }

    let mut pro_list: Vec<ProStats> = pro_map.into_values().collect();
    pro_list.sort_by(|a, b| {
        a.teamname
            .cmp(&b.teamname)
            .then_with(|| role_order(&a.position).cmp(&role_order(&b.position)))
    });

    let team_list: Vec<Team> = team_map.into_values().collect();

    Ok((pro_list, team_list))
}

// Helper: Defines an order for player roles.
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