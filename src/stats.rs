use csv::WriterBuilder;
use serde::{Deserialize, Deserializer};
use std::{collections::HashMap, error::Error, fs::File};

///A struct that stores the stats and fantasy points
///of each pro player
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

///Struct for each LCK Team, stores their wins and names
#[derive(Debug, Default)]
pub struct Team {
    teamname: String,
    wins: u32,
}

impl Team {
    ///Constructor for Team. Builds a new Team with zero wins
    pub fn new(teamname: String) -> Self {
        Self { teamname, wins: 0 }
    }
    // New team for testing
    #[cfg(test)]
    pub fn new_with_wins(teamname: String, wins: u32) -> Self {
        Self { teamname, wins }
    }

    ///Getter for Team. Returns teamname
    ///
    /// # Examples
    ///
    ///  ```
    ///  example_team.teamname();
    ///  "Dplus Kia"
    ///  ```
    pub fn teamname(&self) -> &str {
        &self.teamname
    }

    ///Getter for Team. Returns wins
    ///
    /// # Examples
    ///
    ///  ```
    ///  example_team.get_result();
    ///  "12"
    ///  ```
    pub fn get_result(&self) -> &u32 {
        &self.wins
    }

    ///Prints all of the team names and wins out of a
    ///Vector of Teams
    /// # Examples
    ///
    ///  ```
    ///  Team::print_teams(teams)
    ///  Team Name      Wins
    ///  KT Rolster     7
    ///  Dplus Kia      12
    ///  etc...
    ///  ```    
    pub fn print_teams(teams: &[Team]) {
        println!("{:<20}Wins", "Team Name");
        for team in teams {
            println!("{:<20}{}", team.teamname, team.wins);
        }
    }
}

impl ProStats {
    //Empty ProStat but only using it in tests since it shouldn't be needed
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

    ///Getter for ProStats. Returns playername
    ///
    /// # Examples
    ///
    ///  ```
    ///  example_player.playername()
    ///  Siwoo
    ///  ```
    pub fn playername(&self) -> &str {
        &self.playername
    }

    ///Getter for ProStats. Returns weeklypoints
    ///
    /// # Examples
    ///
    ///  ```
    ///  example_player.weeklypoints()
    ///  24.5
    ///  ```
    pub fn weeklypoints(&self) -> f64 {
        self.weeklypoints
    }

    ///Getter for ProStats. Returns position
    ///
    /// # Examples
    ///
    ///  ```
    ///  example_player.position()
    ///  top
    ///  ```
    pub fn position(&self) -> &str {
        &self.position
    }

    ///Adds the current totalpoints with weeklypoints
    ///
    /// # Examples
    ///
    ///  ```
    ///  example_player.update_totalpoints()
    ///  24.00 += 25.00
    ///  totalpoints = 29.00
    ///  ```
    pub fn update_totalpoints(&mut self) {
        self.totalpoints += self.weeklypoints;
    }

    ///Calculates pro player's points based off role and stats
    ///
    /// # Examples
    ///
    ///  ```
    ///  example_player.calculate_points()
    ///  (example_player is top lane)
    ///  weeklypoints = (3 * 0.4) + (2 * 0.2) + (202 * 0.002) - (2 * 0.15)
    ///  weeklypoints = 1.7
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
    }

    ///Prints all of the pro players
    ///
    /// # Examples
    ///
    ///  ```
    ///  Name                     Kills    Deaths   Assists     CS    Weekly     Total
    ///  BNK FEARX
    ///  Clear                       20        48        59   2885     18.36     18.36
    ///  Raptor                      32        51        67   2206     22.98     22.98
    ///  VicLa                       26        49        67   3261     15.54     15.54
    ///  Diable                      59        34        52   3710     30.16     30.16
    ///  Kellin                       8        54        96    335     22.01     22.01
    ///  etc...
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

    ///Searches through ProStats vector for the inputted playername
    ///
    /// # Examples
    ///
    ///  ```
    ///  ProStats::print_by_name(players, Clear);
    ///  Name                     Kills    Deaths   Assists     CS    Weekly     Total
    ///  Clear                       20        48        59   2885     18.36     18.36
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

    ///Searches through ProStats vector for the inputted playername, but DOES NOT print
    ///header. Used more as a helper function for Coaches.
    ///
    /// # Examples
    ///
    ///  ```
    ///  ProStats::print_by_name_no_header(players, Clear);
    ///  Clear                       20        48        59   2885     18.36     18.36
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

    ///Searches through ProStats vector for the inputted teamname
    ///
    /// # Examples
    ///
    ///  ```
    ///  ProStats::print_by_team(players, BNK FearX);
    ///  Name                     Kills    Deaths   Assists     CS    Weekly     Total
    ///  BNK FEARX
    ///  Clear                       20        48        59   2885     18.36     18.36
    ///  Raptor                      32        51        67   2206     22.98     22.98
    ///  VicLa                       26        49        67   3261     15.54     15.54
    ///  Diable                      59        34        52   3710     30.16     30.16
    ///  Kellin                       8        54        96    335     22.01     22.01
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

    ///Prints the top weekly scorers per role
    ///
    /// # Examples
    ///
    ///  ```
    ///  ProStats::print_top_role(players);
    ///  Top Players by Role:
    ///  Name                     Kills    Deaths   Assists     CS    Weekly     Total
    ///  Morgan                   10043        40        71   3860   4033.11   4033.11
    ///  GIDEON                      69        55       199   5481     70.12     70.12
    ///  Zeka                        93        51       148   6446     58.87     58.87
    ///  Viper                      134        48       146   6940     76.32     76.32
    ///  Delight                     32        77       242    916     65.16     65.16
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

    ///Helper function that prints the header for each of the pro
    ///stats printer functions
    ///
    /// # Examples
    ///
    ///  ```
    ///  print_header()
    ///  Name                     Kills    Deaths   Assists     CS    Weekly     Total
    ///  ```
    pub fn print_header() {
        println!(
            "{:<20}{:>10}{:>10}{:>10}{:>7}{:>10}{:>10}",
            "Name", "Kills", "Deaths", "Assists", "CS", "Weekly", "Total"
        );
    }

    ///Helper function that prints the pro stats for a player
    ///
    /// # Examples
    ///
    ///  ```
    ///  GIDEON                      69        55       199   5481     70.12     70.12
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

///Turns an empty string to a f64 in order to avoid errors when deserializing
fn default_empty_string_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<&str> = Option::deserialize(deserializer)?;
    Ok(s.unwrap_or("").parse::<f64>().unwrap_or(0.0))
}

///Parses through both pro_list.csv and data.csv and store the data into two vectors
///One vector is for ProStats for all the pro players and the other is for Teams
///for all the Teams in the LCK. All the stats are deserialized and stored into their
///structs. They are stored in alphabetical order and for ProStats, they are stored in
///order of role (top -> jng -> mid -> bot -> sup)
pub fn store_player() -> Result<(Vec<ProStats>, Vec<Team>), Box<dyn Error>> {
    //Deserialize the First Pro List
    let file_players = File::open("pro_list.csv")?;
    let mut rdr_players = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file_players);

    let mut pro_map: HashMap<String, ProStats> = HashMap::new();
    let mut team_map: HashMap<String, Team> = HashMap::new();

    for result in rdr_players.deserialize() {
        let record: ProStats = result?;
        pro_map.insert(record.playername.clone(), record);
    }

    //Deserialize the actual data
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

///Stores all the pro players to pro_list.csv.
///Resets everything to 0 except for totalpoints in order
///to reset for the next week.
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

///Matches a role with a usize to help with sorting by role
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
