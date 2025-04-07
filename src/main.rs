use std::io::{self, Write};

mod coaches;
mod stats;

use coaches::Coach;
use stats::{store_player, ProStats, Team};

//Main
fn main() {
    // Load players and teams from CSV files.
    let (mut players, teams) = match store_player() {
        Ok(data) => data,
        Err(err) => {
            println!("Error loading data: {}", err);
            return;
        }
    };

    // Initialize the coaches list
    let mut coaches = match Coach::load_from_csv("coaches.csv") {
        Ok(c) => c,
        Err(err) => {
            println!("Error loading coaches: {}", err);
            Vec::new()
        }
    };

    // Run the main menu loop.
    run_menu(&mut players, &teams, &mut coaches);
}

// MENUS

//Main menu
fn run_menu(players: &mut [ProStats], teams: &[Team], coaches: &mut Vec<Coach>) {
    loop {
        println!("\nMain Menu:");
        println!("1: Coach Options");
        println!("2: Pro Stats");
        println!("3: View All Teams");
        println!("4: Utility Options (Save Here!)");
        println!("0: Quit");
        print!("Enter your choice (as an integer): ");
        io::stdout().flush().unwrap();

        let mut choice_str = String::new();
        io::stdin().read_line(&mut choice_str).unwrap();
        let choice_result = choice_str.trim().parse::<i32>();

        match choice_result {
            Ok(choice) => match choice {
                1 => {
                    coach_options_menu(coaches, players, teams); // Pass Vec directly
                }
                2 => {
                    pro_stats_menu(players);
                }
                3 => {
                    Team::print_teams(teams);
                }
                4 => {
                    utility_menu(players, coaches); // Pass Vec directly
                }
                0 => {
                    break;
                }
                _ => {
                    println!("Invalid option. Please try again.");
                }
            },
            Err(_) => {
                println!("Invalid input. Please enter a valid integer.");
            }
        }
    }
}

///Coach menu
fn coach_options_menu(coaches: &mut Vec<Coach>, players: &[ProStats], teams: &[Team]) {
    loop {
        println!("\nCoach Options:");
        println!("1: Add a Coach");
        println!("2: Remove a Coach");
        println!("3: Edit Team");
        println!("4: Calculate Coach Points (ONLY DO THIS ONCE!!)");
        println!("5: Print All Coaches");
        println!("6: Print a Coach's Team");
        println!("0: Return to Main Menu");
        print!("Enter your choice (as an integer): ");
        io::stdout().flush().unwrap();

        let mut choice_str = String::new();
        io::stdin().read_line(&mut choice_str).unwrap();
        let choice_result = choice_str.trim().parse::<i32>();

        match choice_result {
            Ok(choice) => match choice {
                1 => {
                    let new_coach = build_new_coach(players, teams);
                    coaches.push(new_coach); // This now works because `coaches` is a Vec
                    println!("\nNew coach successfully added!");
                }
                2 => {
                    remove_coach(coaches); // Works because `remove_coach` expects a Vec
                }
                3 => {
                    edit_team(coaches, players, teams);
                }
                4 => {
                    calculate_coach_points(players, teams, coaches);
                }
                5 => {
                    println!("\nAll Coaches:");
                    Coach::print_all(coaches);
                }
                6 => {
                    print_coach_team(coaches, players);
                }
                0 => {
                    break;
                }
                _ => {
                    println!("Invalid option. Please try again.");
                }
            },
            Err(_) => {
                println!("Invalid input. Please enter a valid integer.");
            }
        }
    }
}

///Pro Stats menu
fn pro_stats_menu(players: &mut [ProStats]) {
    loop {
        println!("\nPro Stats Menu:");
        println!("1: Print All");
        println!("2: Print by Name");
        println!("3: Print by Team");
        println!("4: Print Top Scores Per Role");
        println!("5: Update Total Points (ONLY DO THIS ONCE!!)");
        println!("0: Return to Main Menu");
        print!("Enter your choice (as an integer): ");
        io::stdout().flush().unwrap();

        let mut choice_str = String::new();
        io::stdin().read_line(&mut choice_str).unwrap();
        let choice_result = choice_str.trim().parse::<i32>();

        match choice_result {
            Ok(choice) => match choice {
                1 => {
                    println!("\nAll Players:");
                    ProStats::print_all(players);
                }
                2 => {
                    print_pro_stat_by_name(players);
                }
                3 => {
                    print_pro_stat_by_team(players);
                }
                4 => {
                    ProStats::print_top_role(players);
                }
                5 => {
                    update_all_player_points(players);
                    println!("Player total points updated.");
                }
                0 => {
                    break;
                }
                _ => {
                    println!("Invalid option. Please try again.");
                }
            },
            Err(_) => {
                println!("Invalid input. Please enter a valid integer.");
            }
        }
    }
}

///Utility menu
fn utility_menu(players: &mut [ProStats], coaches: &mut [Coach]) {
    loop {
        println!("\nUtility Options:");
        println!("1: Save Players to CSV");
        println!("2: Save Coaches to CSV");
        println!("0: Return To Main Menu");
        print!("Enter your choice (as an integer): ");
        io::stdout().flush().unwrap();

        let mut choice_str = String::new();
        io::stdin().read_line(&mut choice_str).unwrap();
        let choice_result = choice_str.trim().parse::<i32>();

        match choice_result {
            Ok(choice) => match choice {
                1 => {
                    if let Err(err) = stats::write_players_to_csv(players, "pro_list.csv") {
                        eprintln!("Error saving players to CSV: {}", err);
                    } else {
                        println!("Players successfully saved to pro_list.csv!");
                    }
                }
                2 => {
                    // Save Coaches to coaches.csv
                    if let Err(err) = Coach::write_to_csv(coaches, "coaches.csv") {
                        eprintln!("Error saving coaches to CSV: {}", err);
                    } else {
                        println!("Coaches successfully saved to coaches.csv!");
                    }
                }
                0 => {
                    // Exit option
                    break;
                }
                _ => {
                    println!("Invalid option. Please try again.");
                }
            },
            Err(_) => {
                println!("Invalid input. Please enter a valid integer.");
            }
        }
    }
}

//Menu Options

///Prompts the user to build a new coach with player inputs.
fn build_new_coach(players: &[ProStats], teams: &[Team]) -> Coach {
    let mut input = String::new();

    // Prompt for coach name.
    print!("Enter Coach Name: ");
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let coach_name = input.trim().to_string();

    // Prompt for each role
    let top = prompt_for_valid_player("top", players);
    let jng = prompt_for_valid_player("jng", players);
    let mid = prompt_for_valid_player("mid", players);
    let bot = prompt_for_valid_player("bot", players);
    let sup = prompt_for_valid_player("sup", players);

    // Prompt for the LCK Team name with validation.
    let team = prompt_for_valid_team(teams);

    // Create and return the new coach. Weekly and total points start as 0.0.
    Coach::new(coach_name, top, jng, mid, bot, sup, team)
}

///Prompts for a player name for the given role until a valid one is entered.
fn prompt_for_valid_player(expected_role: &str, players: &[ProStats]) -> String {
    let mut input = String::new();
    loop {
        print!("Enter {} player name: ", expected_role);
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let candidate = input.trim().to_string();
        if players.iter().any(|p| {
            p.playername().eq_ignore_ascii_case(&candidate)
                && p.position().eq_ignore_ascii_case(expected_role)
        }) {
            return candidate;
        } else {
            println!(
                "Player '{}' not found with role '{}'. Please try again.",
                candidate, expected_role
            );
        }
    }
}

///Prompts for a LCK team name for the given role until a valid one is entered.
fn prompt_for_valid_team(teams: &[Team]) -> String {
    let mut input = String::new();

    loop {
        print!("Enter LCK Team Name: ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let team_name = input.trim();
        if teams
            .iter()
            .any(|t| t.teamname().eq_ignore_ascii_case(team_name))
        {
            return team_name.to_string();
        } else {
            println!("Invalid team name. Please enter a valid LCK team name.");
        }
    }
}

///Removes a coach by name.
fn remove_coach(coaches: &mut Vec<Coach>) {
    print!("Enter the name of the coach to remove: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let target_name = input.trim();

    if Coach::remove_coach_by_name(coaches, target_name) {
        println!("Coach '{}' has been removed successfully.", target_name);
    } else {
        println!("No coach found with the name '{}'.", target_name);
    }
}

///Prints a coach's team by name.
fn print_coach_team(coaches: &[Coach], players: &[ProStats]) {
    print!("Enter the coach's name to view their team: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let target_coach = input.trim();

    Coach::print_team_by_coach(coaches, players, target_coach);
}

///Coaches can edit their team given a variety of options
fn edit_team(coaches: &mut [Coach], players: &[ProStats], teams: &[Team]) {
    let mut input = String::new();
    print!("Enter the coach's name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let coach_name = input.trim();

    // Find the coach
    if let Some(coach) = coaches
        .iter_mut()
        .find(|c| c.get_coach_name().eq_ignore_ascii_case(coach_name))
    {
        // Prompt for what to edit: role or LCK team
        loop {
            println!("\nWhat do you want to edit?");
            println!("1: Top");
            println!("2: Jungle");
            println!("3: Mid");
            println!("4: Bot");
            println!("5: Support");
            println!("6: LCK Team");
            print!("Enter your choice (as an integer): ");
            io::stdout().flush().unwrap();

            let mut choice_str = String::new();
            io::stdin().read_line(&mut choice_str).unwrap();
            let choice_result = choice_str.trim().parse::<i32>();

            match choice_result {
                Ok(1) => {
                    coach.set_role("top", prompt_for_valid_player("top", players));
                    break;
                }
                Ok(2) => {
                    coach.set_role("jungle", prompt_for_valid_player("jungle", players));
                    break;
                }
                Ok(3) => {
                    coach.set_role("mid", prompt_for_valid_player("mid", players));
                    break;
                }
                Ok(4) => {
                    coach.set_role("bot", prompt_for_valid_player("bot", players));
                    break;
                }
                Ok(5) => {
                    coach.set_role("support", prompt_for_valid_player("sup", players));
                    break;
                }
                Ok(6) => {
                    coach.set_team(prompt_for_valid_team(teams));
                    break;
                }
                _ => {
                    println!("Invalid option. Please try again.");
                }
            }
        }
    } else {
        println!("Coach not found. Please try again.");
    }
}

///Takes user input for pro player and prints that player
fn print_pro_stat_by_name(players: &[ProStats]) {
    print!("Enter the player's name: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let player_name = input.trim();

    ProStats::print_by_name(players, player_name);
}

///Takes user input for team and prints that team
fn print_pro_stat_by_team(players: &[ProStats]) {
    print!("Enter the team's name: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let team_name = input.trim();

    ProStats::print_by_team(players, team_name);
}

///Updates the totalpoints for pro players
fn update_all_player_points(players: &mut [ProStats]) {
    for player in players.iter_mut() {
        player.update_totalpoints();
    }
    println!("Total points for all players have been updated.");
}

///Updates weekly and totalpoints for Coaches.
fn calculate_coach_points(players: &[ProStats], teams: &[Team], coaches: &mut [Coach]) {
    for coach in coaches.iter_mut() {
        coach.set_weekly(players, teams);
        coach.set_total();
    }
}

//Tests

#[cfg(test)]
mod tests {
    use crate::coaches::Coach;
    use crate::stats::ProStats;
    use crate::stats::Team;

    #[test]
    fn test_set_weekly_and_total() {
        let players = vec![
            ProStats::new(
                "Player1".to_string(),
                "top".to_string(),
                "Team1".to_string(),
                0.0,
                0.0,
                0.0,
                0.0,
                "LCK".to_string(),
                20.0,
                0.0,
            ),
            ProStats::new(
                "Player2".to_string(),
                "jng".to_string(),
                "Team1".to_string(),
                0.0,
                0.0,
                0.0,
                0.0,
                "LCK".to_string(),
                15.0,
                0.0,
            ),
        ];
        let teams = vec![Team::new_with_wins("Team1".to_string(), 2)];
        let mut coach = Coach::new(
            "Coach1".to_string(),
            "Player1".to_string(),
            "Player2".to_string(),
            "Player3".to_string(),
            "Player4".to_string(),
            "Player5".to_string(),
            "Team1".to_string(),
        );

        coach.set_weekly(&players, &teams);
        assert_eq!(coach.weeklypoints(), 40.0);

        coach.set_total();
        assert_eq!(coach.totalpoints(), 40.0);
    }
    #[test]
    fn test_set_weekly_missing_players() {
        let players = vec![ProStats::new(
            "Player1".to_string(),
            "top".to_string(),
            "Team1".to_string(),
            0.0,
            0.0,
            0.0,
            0.0,
            "LCK".to_string(),
            20.0,
            0.0,
        )];
        let teams = vec![Team::new_with_wins("Team1".to_string(), 2)];
        let mut coach = Coach::new(
            "Coach1".to_string(),
            "Player1".to_string(),
            "MissingPlayer".to_string(),
            "Player3".to_string(),
            "Player4".to_string(),
            "Player5".to_string(),
            "Team1".to_string(),
        );

        coach.set_weekly(&players, &teams);
        assert_eq!(coach.weeklypoints(), 20.0 + 0.0 + (2.0 * 2.5)); // Only Player1 and team points
    }

    #[test]
    fn test_set_weekly_missing_team() {
        let players = vec![
            ProStats::new(
                "Player1".to_string(),
                "top".to_string(),
                "Team1".to_string(),
                0.0,
                0.0,
                0.0,
                0.0,
                "LCK".to_string(),
                20.0,
                0.0,
            ),
            ProStats::new(
                "Player2".to_string(),
                "jng".to_string(),
                "Team1".to_string(),
                0.0,
                0.0,
                0.0,
                0.0,
                "LCK".to_string(),
                15.0,
                0.0,
            ),
        ];
        let teams = vec![]; // No teams in the list
        let mut coach = Coach::new(
            "Coach1".to_string(),
            "Player1".to_string(),
            "Player2".to_string(),
            "Player3".to_string(),
            "Player4".to_string(),
            "Player5".to_string(),
            "MissingTeam".to_string(), // Team not in `teams`
        );

        coach.set_weekly(&players, &teams);
        assert_eq!(coach.weeklypoints(), 20.0 + 15.0 + 0.0); // Only player points
    }

    #[test]
    fn test_remove_coach_by_name() {
        let mut coaches = vec![
            Coach::new(
                "Coach1".to_string(),
                "Player1".to_string(),
                "Player2".to_string(),
                "Player3".to_string(),
                "Player4".to_string(),
                "Player5".to_string(),
                "Team1".to_string(),
            ),
            Coach::new(
                "Coach2".to_string(),
                "Player6".to_string(),
                "Player7".to_string(),
                "Player8".to_string(),
                "Player9".to_string(),
                "Player10".to_string(),
                "Team2".to_string(),
            ),
        ];

        let result = Coach::remove_coach_by_name(&mut coaches, "Coach1");
        assert!(result); // Successfully removed
        assert_eq!(coaches.len(), 1);
        assert_eq!(coaches[0].get_coach_name(), "Coach2");

        let result_nonexistent = Coach::remove_coach_by_name(&mut coaches, "NonexistentCoach");
        assert!(!result_nonexistent); // No coach removed
        assert_eq!(coaches.len(), 1); // Length remains the same
    }

    #[test]
    fn test_print_by_team() {
        let players = vec![
            ProStats::new(
                "Player1".to_string(),
                "top".to_string(),
                "Team1".to_string(),
                0.0,
                0.0,
                0.0,
                0.0,
                "LCK".to_string(),
                10.0,
                50.0,
            ),
            ProStats::new(
                "Player2".to_string(),
                "jng".to_string(),
                "Team2".to_string(),
                0.0,
                0.0,
                0.0,
                0.0,
                "LCK".to_string(),
                15.0,
                30.0,
            ),
        ];

        ProStats::print_by_team(&players, "Team1");
    }

    #[test]
    fn test_empty_players_and_teams() {
        let players: Vec<ProStats> = vec![];
        let teams: Vec<Team> = vec![];
        let mut coach = Coach::new(
            "Coach1".to_string(),
            "Player1".to_string(),
            "Player2".to_string(),
            "Player3".to_string(),
            "Player4".to_string(),
            "Player5".to_string(),
            "Team1".to_string(),
        );

        coach.set_weekly(&players, &teams);
        assert_eq!(coach.weeklypoints(), 0.0); // No points calculated
    }

    #[test]
    fn test_write_to_csv() {
        let mut coaches = vec![Coach::new(
            "Coach1".to_string(),
            "Player1".to_string(),
            "Player2".to_string(),
            "Player3".to_string(),
            "Player4".to_string(),
            "Player5".to_string(),
            "Team1".to_string(),
        )];

        let file_path = "test_coaches.csv";
        let result = Coach::write_to_csv(&mut coaches, file_path);
        assert!(result.is_ok());

        std::fs::remove_file(file_path).unwrap(); // Clean up test file
    }
}
