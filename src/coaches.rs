use serde::Deserialize;
use std::{error::Error, fs::File};

use crate::stats::Team;

///Struct for Coach. This stores the Coach's player names, coach name,
///lck team and both weekly/total points
#[derive(Debug, Deserialize)]
pub struct Coach {
    coach_name: String,
    top: String,
    jng: String,
    mid: String,
    bot: String,
    sup: String,
    team: String,
    weeklypoints: f64,
    totalpoints: f64,
}

impl Coach {
    ///Constructor that builds a new empty Coach
    pub fn new(
        coach_name: String,
        top: String,
        jng: String,
        mid: String,
        bot: String,
        sup: String,
        team: String,
    ) -> Self {
        Self {
            coach_name,
            top,
            jng,
            mid,
            bot,
            sup,
            team,
            weeklypoints: 0.0,
            totalpoints: 0.0,
        }
    }
    #[cfg(test)]
    pub fn weeklypoints(&self) -> f64 {
        self.weeklypoints
    }

    #[cfg(test)]
    pub fn totalpoints(&self) -> f64 {
        self.totalpoints
    }

    ///Getter for Coach. Returns coach_name
    ///
    /// # Examples
    ///
    ///  ```
    ///  example_coach.teamname();
    ///  "fraudler1"
    ///  ```
    pub fn get_coach_name(&self) -> &str {
        &self.coach_name
    }

    ///Getter for Coach. Returns team
    ///
    /// # Examples
    ///
    ///  ```
    ///  example_coach.get_team();
    ///  "Dplus Kia"
    ///  ```
    pub fn get_team(&self) -> &str {
        &self.team
    }

    ///Getter for Coach. Returns totalpoints
    ///
    /// # Examples
    ///
    ///  ```
    ///  example_coach.get_totalpoints();
    ///  143.23
    ///  ```
    pub fn get_totalpoints(&self) -> f64 {
        self.totalpoints
    }

    ///Setter for Coach. Sets a inputted team for Coach
    ///
    /// # Examples
    ///
    ///  ```
    ///  example_coach.get_totalpoints("KT Rolster");
    ///  example_coach.team = KT Rolster
    ///  ```
    pub fn set_team(&mut self, new_team: String) {
        self.team = new_team;
    }

    ///Sets the pro player according to player's role. It they are attempting
    ///to store to store a pro in the wrong role, it will output error
    ///
    /// # Examples
    ///
    ///  ```text
    ///  coach.set_role("top", siwoo);
    ///  Coach's top laner set to Siwoo.
    ///  OR
    ///  coach.set_role("jng", siwoo);
    ///  Invalid role jng
    ///  ```
    pub fn set_role(&mut self, role: &str, player_name: String) {
        match role.to_lowercase().as_str() {
            "top" => self.top = player_name,
            "jungle" | "jng" => self.jng = player_name,
            "mid" => self.mid = player_name,
            "bot" => self.bot = player_name,
            "support" | "sup" => self.sup = player_name,
            _ => println!("Invalid role: {}", role),
        }
    }

    ///Sets weekly score for Coach by finding all the players/teams in the coach's team and totalling it.
    pub fn set_weekly(&mut self, pro_list: &[crate::stats::ProStats], team_list: &[Team]) {
        fn find_player_weekly(player_name: &str, pro_list: &[crate::stats::ProStats]) -> f64 {
            // Lowercase the coach's player name for comparison.
            let target = player_name.to_lowercase();
            if let Some(player) = pro_list
                .iter()
                .find(|p| p.playername().to_lowercase() == target)
            {
                player.weeklypoints()
            } else {
                0.0
            }
        }

        fn find_team_weekly(team_name: &str, team_list: &[Team]) -> u32 {
            let target = team_name.to_lowercase();
            if let Some(team) = team_list
                .iter()
                .find(|t| t.teamname().to_lowercase() == target)
            {
                *team.get_result() // Dereference to return the u32 value
            } else {
                0
            }
        }

        let top_weekly = find_player_weekly(&self.top, pro_list);
        let jng_weekly = find_player_weekly(&self.jng, pro_list);
        let mid_weekly = find_player_weekly(&self.mid, pro_list);
        let bot_weekly = find_player_weekly(&self.bot, pro_list);
        let sup_weekly = find_player_weekly(&self.sup, pro_list);

        let team_weekly = find_team_weekly(&self.team, team_list);

        self.weeklypoints = top_weekly
            + jng_weekly
            + mid_weekly
            + bot_weekly
            + sup_weekly
            + (team_weekly as f64 * 2.5);
    }

    ///Adds the current totalpoints with weeklypoints, updating totalpoints
    pub fn set_total(&mut self) {
        self.totalpoints += self.weeklypoints;
    }

    ///Loads coaches.csv to deseralize and store
    pub fn load_from_csv(file_path: &str) -> Result<Vec<Coach>, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file);

        let mut coaches = Vec::new();

        for result in rdr.deserialize() {
            let coach: Coach = result?;
            coaches.push(coach);
        }

        Ok(coaches)
    }

    ///Removes a inputted Coach from the coaches' vector
    ///
    /// # Examples
    ///
    ///  ```
    ///  Current Coaches: fraduler1, badname, notreal
    ///  remove_coach_by_name(coaches, fraduler1)
    ///  Current Coaches: badname, notreal
    ///  ```
    pub fn remove_coach_by_name(coaches: &mut Vec<Coach>, name: &str) -> bool {
        if let Some(pos) = coaches
            .iter()
            .position(|c| c.coach_name.eq_ignore_ascii_case(name))
        {
            coaches.remove(pos);
            true
        } else {
            false
        }
    }

    ///Prints inputted Coach's team and scores
    ///
    /// # Examples
    ///
    ///  ```
    ///  Name                     Kills    Deaths   Assists     CS    Weekly     Total
    ///  fraudler1                                                      0.00      0.00
    ///  Siwoo                       54        46       107   4113     44.33     44.33
    ///  Peanut                      58        66       212   4697     65.10     65.10
    ///  Faker                       51        37       103   4607     35.37     35.37
    ///  Jiwoo                      111        63       138   7441     63.67     63.67
    ///  Delight                     32        77       242    916     65.16     65.16
    ///  gen.g
    ///  ```
    pub fn print_team_by_coach(
        coaches: &[Coach],
        players: &[crate::stats::ProStats],
        target_coach: &str,
    ) {
        if let Some(coach) = coaches
            .iter()
            .find(|c| c.coach_name.eq_ignore_ascii_case(target_coach))
        {
            crate::stats::ProStats::print_header();
            println!(
                "{:<20}{:>10}{:>10}{:>10}{:>7}{:>10.2}{:>10.2}",
                coach.coach_name, "", "", "", "", coach.weeklypoints, coach.totalpoints
            );
            crate::stats::ProStats::print_by_name_no_header(players, &coach.top);
            crate::stats::ProStats::print_by_name_no_header(players, &coach.jng);
            crate::stats::ProStats::print_by_name_no_header(players, &coach.mid);
            crate::stats::ProStats::print_by_name_no_header(players, &coach.bot);
            crate::stats::ProStats::print_by_name_no_header(players, &coach.sup);
            println!("{}", coach.team);
        } else {
            println!("Coach not found: {}", target_coach);
        }
        println!();
    }

    ///Prints all the coaches and their weekly/total points
    ///
    /// # Examples
    ///
    ///  ```
    ///  All Coaches:
    ///  Coach Name:                Weekly Points        Total Points
    ///  fraudler1                           0.00                0.00
    ///  badname                             0.00                0.00
    ///  ```
    pub fn print_all(coaches: &[Coach]) {
        // Print header line.
        println!(
            "{:<20}{:>20}{:>20}",
            "Coach Name:", "Weekly Points", "Total Points"
        );
        // Print each coach's data.
        for coach in coaches {
            println!(
                "{:<20}{:>20.2}{:>20.2}",
                coach.coach_name, coach.weeklypoints, coach.totalpoints
            );
        }
        println!();
    }

    ///Store ALL of the Coaches' data into coaches.csv. Resets weekly points to 0 to prepare for
    ///next week
    pub fn write_to_csv(
        coaches: &mut [Coach],
        file_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use csv::WriterBuilder;
        use std::fs::File;

        let file = File::create(file_path)?;
        let mut wtr = WriterBuilder::new().has_headers(true).from_writer(file);

        wtr.write_record([
            "coach_name",
            "top",
            "jng",
            "mid",
            "bot",
            "sup",
            "team",
            "weeklypoints",
            "totalpoints",
        ])?;

        for coach in coaches.iter_mut() {
            wtr.write_record([
                coach.get_coach_name(),
                &coach.top,
                &coach.jng,
                &coach.mid,
                &coach.bot,
                &coach.sup,
                coach.get_team(),
                "0",
                &coach.get_totalpoints().to_string(),
            ])?;
        }

        wtr.flush()?;
        println!("Coaches successfully written to {}", file_path);
        Ok(())
    }
}
