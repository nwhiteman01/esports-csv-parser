use csv::WriterBuilder;
use serde::{Deserialize, Deserializer};
use std::{collections::HashMap, error::Error, fs::File};

///Determines if the center value will be true or false depending on left,
///center, and right values
///
/// # Examples
///
///  ```
///  bits = [true, true, false]
///  set bits = true
///  ```
#[derive(Debug, Deserialize, Clone)]
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
    #[serde(
        rename = "total cs",
        default,
        deserialize_with = "default_empty_string_to_f64"
    )]
    total_cs: f64,
    league: String,
    #[serde(default)]
    weeklypoints: f64,
    #[serde(default)]
    totalpoints: f64,
    #[serde(default)]
    result: u32,
}

///Determines if the center value will be true or false depending on left,
///center, and right values
///
/// # Examples
///
///  ```
///  bits = [true, true, false]
///  set bits = true
///  ```
#[derive(Debug, Default)]
pub struct Team {
    teamname: String,
    wins: u32,
}

impl Team {
    // Create a new team with a given name and zero initial wins
    ///Determines if the center value will be true or false depending on left,
    ///center, and right values
    ///
    /// # Examples
    ///
    ///  ```
    ///  bits = [true, true, false]
    ///  set bits = true
    ///  ```
    pub fn new(teamname: String) -> Self {
        Self { teamname, wins: 0 }
    }
    // New team for testing
    #[cfg(test)]
    pub fn new_with_wins(teamname: String, wins: u32) -> Self {
        Self { teamname, wins }
    }

    ///Determines if the center value will be true or false depending on left,
    ///center, and right values
    ///
    /// # Examples
    ///
    ///  ```
    ///  bits = [true, true, false]
    ///  set bits = true
    ///  ```
    pub fn teamname(&self) -> &str {
        &self.teamname
    }

    ///Determines if the center value will be true or false depending on left,
    ///center, and right values
    ///
    /// # Examples
    ///
    ///  ```
    ///  bits = [true, true, false]
    ///  set bits = true
    ///  ```
    pub fn get_result(&self) -> &u32 {
        &self.wins
    }

    ///Determines if the center value will be true or false depending on left,
    ///center, and right values
    ///
    /// # Examples
    ///
    ///  ```
    ///  bits = [true, true, false]
    ///  set bits = true
    ///  ```    
    pub fn print_teams(teams: &[Team]) {
        println!("{:<20}Wins", "Team Name");
        for team in teams {
            println!("{:<20}{}", team.teamname, team.wins);
        }
    }
}

impl ProStats {
    #[cfg(test)]
    pub fn new(
        playername: String,
        position: String,
        teamname: String,
        kills: f64,
        deaths: f64,
        assists: f64,
        total_cs: f64,
        league: String,
        weeklypoints: f64,
        totalpoints: f64,
    ) -> Self {
        Self {
            playername,
            position,
            teamname,
            kills,
            deaths,
            assists,
            total_cs,
            league,
            weeklypoints,
            totalpoints,
            result: 0,
        }
    }

    ///Determines if the center value will be true or false depending on left,
    ///center, and right values
    ///
    /// # Examples
    ///
    ///  ```
    ///  bits = [true, true, false]
    ///  set bits = true
    ///  ```
    pub fn playername(&self) -> &str {
        &self.playername
    }

    ///Determines if the center value will be true or false depending on left,
    ///center, and right values
    ///
    /// # Examples
    ///
    ///  ```
    ///  bits = [true, true, false]
    ///  set bits = true
    ///  ```
    pub fn weeklypoints(&self) -> f64 {
        self.weeklypoints
    }

    ///Determines if the center value will be true or false depending on left,
    ///center, and right values
    ///
    /// # Examples
    ///
    ///  ```
    ///  bits = [true, true, false]
    ///  set bits = true
    ///  ```
    pub fn position(&self) -> &str {
        &self.position
    }

    ///Determines if the center value will be true or false depending on left,
    ///center, and right values
    ///
    /// # Examples
    ///
    ///  ```
    ///  bits = [true, true, false]
    ///  set bits = true
    ///  ```
    pub fn update_totalpoints(&mut self) {
        self.totalpoints += self.weeklypoints;
    }

    ///Determines if the center value will be true or false depending on left,
    ///center, and right values
    ///
    /// # Examples
    ///
    ///  ```
    ///  bits = [true, true, false]
    ///  set bits = true
    ///  ```
    pub fn calculate_points(&mut self) {
        let kills = self.kills;
        let deaths = self.deaths;
        let assists = self.assists;
        let total_cs = self.total_cs;

        if self.position == "top" || self.position == "jng" {
            self.weeklypoints =
                (kills * 0.4) + (assists * 0.2) + (total_cs * 0.002) - (deaths * 0.15);
        } else if self.position == "mid" || self.position == "bot" {
            self.weeklypoints =
                (kills * 0.4) + (assists * 0.15) + (total_cs * 0.0015) - (deaths * 0.2);
        } else {
            self.weeklypoints =
                (kills * 0.3) + (assists * 0.25) + (total_cs * 0.003) - (deaths * 0.1);
        }

        self.weeklypoints = (self.weeklypoints * 100.0).round() / 100.0;
    }

    ///Determines if the center value will be true or false depending on left,
    ///center, and right values
    ///
    /// # Examples
    ///
    ///  ```
    ///  bits = [true, true, false]
    ///  set bits = true
    ///  ```
    pub fn print_all(players: &[ProStats]) {
        Self::print_header();
        let mut current_team = String::new();
        for player in players {
            if player.teamname != current_team {
                if !current_team.is_empty() {
                    println!();
                }
                current_team = player.teamname.clone();
                println!("{}", current_team);
            }
            Self::print_player(player);
        }
    }

    ///Determines if the center value will be true or false depending on left,
    ///center, and right values
    ///
    /// # Examples
    ///
    ///  ```
    ///  bits = [true, true, false]
    ///  set bits = true
    ///  ```
    pub fn print_by_name(players: &[ProStats], name: &str) {
        // Convert target name to lowercase once.
        let target = name.to_lowercase();
        if let Some(player) = players
            .iter()
            .find(|p| p.playername.to_lowercase() == target)
        {
            Self::print_header();
            Self::print_player(player);
        } else {
            println!("Player not found: {}", name);
        }
    }

    ///Determines if the center value will be true or false depending on left,
    ///center, and right values
    ///
    /// # Examples
    ///
    ///  ```
    ///  bits = [true, true, false]
    ///  set bits = true
    ///  ```
    pub fn print_by_name_no_header(players: &[ProStats], name: &str) {
        let target = name.to_lowercase();
        if let Some(player) = players
            .iter()
            .find(|p| p.playername.to_lowercase() == target)
        {
            ProStats::print_player(player);
        } else {
            println!("Player not found: {}", name);
        }
    }

    ///Determines if the center value will be true or false depending on left,
    ///center, and right values
    ///
    /// # Examples
    ///
    ///  ```
    ///  bits = [true, true, false]
    ///  set bits = true
    ///  ```
    pub fn print_by_team(players: &[ProStats], teamname: &str) {
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

    ///Determines if the center value will be true or false depending on left,
    ///center, and right values
    ///
    /// # Examples
    ///
    ///  ```
    ///  bits = [true, true, false]
    ///  set bits = true
    ///  ```
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

    ///Determines if the center value will be true or false depending on left,
    ///center, and right values
    ///
    /// # Examples
    ///
    ///  ```
    ///  bits = [true, true, false]
    ///  set bits = true
    ///  ```
    pub fn print_header() {
        println!(
            "{:<20}{:>10}{:>10}{:>10}{:>7}{:>10}{:>10}",
            "Name", "Kills", "Deaths", "Assists", "CS", "Weekly", "Total"
        );
    }

    ///Determines if the center value will be true or false depending on left,
    ///center, and right values
    ///
    /// # Examples
    ///
    ///  ```
    ///  bits = [true, true, false]
    ///  set bits = true
    ///  ```
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

fn default_empty_string_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<&str> = Option::deserialize(deserializer)?;
    Ok(s.unwrap_or("").parse::<f64>().unwrap_or(0.0))
}

///Determines if the center value will be true or false depending on left,
///center, and right values
///
/// # Examples
///
///  ```
///  bits = [true, true, false]
///  set bits = true
///  ```
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

    let file_new_stats = File::open("data.csv")?;
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
                })
                .or_insert_with(|| record);
        } else if record.playername.is_empty() && record.league == "LCK" {
            team_map
                .entry(record.teamname.clone())
                .and_modify(|team| {
                    team.wins += record.result;
                })
                .or_insert_with(|| {
                    let mut new_team = Team::new(record.teamname.clone());
                    new_team.wins = record.result;
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

///Determines if the center value will be true or false depending on left,
///center, and right values
///
/// # Examples
///
///  ```
///  bits = [true, true, false]
///  set bits = true
///  ```
pub fn write_players_to_csv(players: &[ProStats], file_path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::create(file_path)?;
    let mut wtr = WriterBuilder::new().has_headers(true).from_writer(file);

    wtr.write_record([
        "position",
        "playername",
        "teamname",
        "kills",
        "deaths",
        "assists",
        "total cs",
        "league",
        "weeklypoints",
        "totalpoints",
    ])?;

    for player in players {
        wtr.write_record([
            &player.position,
            &player.playername,
            &player.teamname,
            "0",
            "0",
            "0",
            "0",
            &player.league,
            "0",
            &player.totalpoints.to_string(),
        ])?;
    }

    wtr.flush()?;
    println!("Players successfully written to {}", file_path);
    Ok(())
}

// Role Helper!
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
