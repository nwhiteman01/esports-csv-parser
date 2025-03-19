use serde::Deserialize;
use std::{error::Error, fs::File};

use crate::stats::Team;

///Determines if the center value will be true or false depending on left,
///center, and right values
///
/// # Examples
///
///  ```
///  bits = [true, true, false]
///  set bits = true
///  ```
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
    ///Determines if the center value will be true or false depending on left,
    ///center, and right values
    ///
    /// # Examples
    ///
    ///  ```
    ///  bits = [true, true, false]
    ///  set bits = true
    ///  ```
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

    ///Determines if the center value will be true or false depending on left,
    ///center, and right values
    ///
    /// # Examples
    ///
    ///  ```
    ///  bits = [true, true, false]
    ///  set bits = true
    ///  ```
    pub fn get_coach_name(&self) -> &str {
        &self.coach_name
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

    ///Determines if the center value will be true or false depending on left,
    ///center, and right values
    ///
    /// # Examples
    ///
    ///  ```
    ///  bits = [true, true, false]
    ///  set bits = true
    ///  ```
    pub fn get_team(&self) -> &str {
        &self.team
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
    pub fn set_team(&mut self, new_team: String) {
        self.team = new_team;
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
    pub fn get_totalpoints(&self) -> f64 {
        self.totalpoints
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

    ///Determines if the center value will be true or false depending on left,
    ///center, and right values
    ///
    /// # Examples
    ///
    ///  ```
    ///  bits = [true, true, false]
    ///  set bits = true
    ///  ```
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

    ///Determines if the center value will be true or false depending on left,
    ///center, and right values
    ///
    /// # Examples
    ///
    ///  ```
    ///  bits = [true, true, false]
    ///  set bits = true
    ///  ```
    pub fn set_total(&mut self) {
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

    ///Determines if the center value will be true or false depending on left,
    ///center, and right values
    ///
    /// # Examples
    ///
    ///  ```
    ///  bits = [true, true, false]
    ///  set bits = true
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

    ///Determines if the center value will be true or false depending on left,
    ///center, and right values
    ///
    /// # Examples
    ///
    ///  ```
    ///  bits = [true, true, false]
    ///  set bits = true
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

    ///Determines if the center value will be true or false depending on left,
    ///center, and right values
    ///
    /// # Examples
    ///
    ///  ```
    ///  bits = [true, true, false]
    ///  set bits = true
    ///  ```
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
