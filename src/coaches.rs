use serde::Deserialize;
use std::{error::Error, fs::File};

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
    // Constructor for a new Coach
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

        // --- Getter Methods for private fields ---
        pub fn get_coach_name(&self) -> &str {
            &self.coach_name
        }
        
        pub fn get_weeklypoints(&self) -> f64 {
            self.weeklypoints
        }
        
        pub fn get_totalpoints(&self) -> f64 {
            self.totalpoints
        }

    // Function to load coaches from a CSV file
    pub fn load_from_csv(file_path: &str) -> Result<Vec<Coach>, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let mut rdr = csv::ReaderBuilder::new().has_headers(true).from_reader(file);

        let mut coaches = Vec::new();

        for result in rdr.deserialize() {
            let coach: Coach = result?;
            coaches.push(coach);
        }

        Ok(coaches)
    }

    // Function to set weekly points using the player stats
    pub fn set_weekly(&mut self, pro_list: &[crate::stats::ProStats]) {
        // Helper function to find a player's weekly points, ignoring case.
        fn find_player_weekly(player_name: &str, pro_list: &[crate::stats::ProStats]) -> f64 {
            // Lowercase the coach's player name for comparison.
            let target = player_name.to_lowercase();
            if let Some(player) = pro_list.iter().find(|p| p.playername().to_lowercase() == target) {
                player.weeklypoints()
            } else {
                0.0
            }
        }

        let top_weekly = find_player_weekly(&self.top, pro_list);
        let jng_weekly = find_player_weekly(&self.jng, pro_list);
        let mid_weekly = find_player_weekly(&self.mid, pro_list);
        let bot_weekly = find_player_weekly(&self.bot, pro_list);
        let sup_weekly = find_player_weekly(&self.sup, pro_list);

        self.weeklypoints = top_weekly + jng_weekly + mid_weekly + bot_weekly + sup_weekly;
    }

    pub fn set_total(&mut self) {
        self.totalpoints += self.weeklypoints;
    }

    pub fn remove_coach_by_name(coaches: &mut Vec<Coach>, name: &str) -> bool {
        if let Some(pos) = coaches.iter().position(|c| c.coach_name.eq_ignore_ascii_case(name)) {
            coaches.remove(pos);
            true // Coach was successfully removed
        } else {
            false // No coach with the given name was found
        }
    }

    pub fn print_team_by_coach(coaches: &[Coach], players: &[crate::stats::ProStats], target_coach: &str) {
        if let Some(coach) = coaches.iter().find(|c| c.coach_name.eq_ignore_ascii_case(target_coach)) {
            // Use the header from ProStats.
            // (Assuming crate::stats::print_header() is public. Otherwise, you could manually print your header here.)
            crate::stats::ProStats::print_header();
            println!(
                "{:<20}{:>10}{:>10}{:>10}{:>7}{:>10.2}{:>10.2}",
                coach.coach_name,
                "",   // Kills (blank)
                "",   // Deaths (blank)
                "",   // Assists (blank)
                "",   // CS (blank)
                coach.weeklypoints,
                coach.totalpoints
            );
            crate::stats::ProStats::print_by_name_no_header(players, &coach.top);
            crate::stats::ProStats::print_by_name_no_header(players, &coach.jng);
            crate::stats::ProStats::print_by_name_no_header(players, &coach.mid);
            crate::stats::ProStats::print_by_name_no_header(players, &coach.bot);
            crate::stats::ProStats::print_by_name_no_header(players, &coach.sup);
        } else {
            println!("Coach not found: {}", target_coach);
        }
        println!("");
    }

    // Function to print all coaches
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
        println!("");
    }
}
