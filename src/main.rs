use std::io::{self, Write};

mod stats;
mod coaches;

use stats::{store_player, ProStats, Team};
use coaches::Coach;

fn main() {
    // Load players and teams from your CSV files.
    let (players, teams) = match store_player() {
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
    run_menu(&players, &teams, &mut coaches);
}


/// Displays the main menu for accessing coach and pro stats options or quitting.
fn run_menu(players: &[ProStats], teams: &[Team], coaches: &mut Vec<Coach>) {
    loop {
        println!("\nMain Menu:");
        println!("1: Coach Options");
        println!("2: Pro Stats");
        println!("3: View All Teams");
        println!("4: Quit");
        print!("Enter your choice (as an integer): ");
        io::stdout().flush().unwrap();

        let mut choice_str = String::new();
        io::stdin().read_line(&mut choice_str).unwrap();
        let choice_result = choice_str.trim().parse::<i32>();

        match choice_result {
            Ok(choice) => match choice {
                1 => {
                    coach_options_menu(coaches, players, teams); // Pass `teams` here
                }
                2 => {
                    pro_stats_menu(players);
                }
                3 => {
                    Team::print_teams(teams);
                }
                4 => {
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


fn coach_options_menu(coaches: &mut Vec<Coach>, players: &[ProStats], teams: &[Team]) {
    loop {
        println!("\nCoach Options:");
        println!("1: Add a Coach");
        println!("2: Remove a Coach");
        println!("3: Print All Coaches");
        println!("4: Print a Coach's Team");
        println!("5: Return to Main Menu");
        print!("Enter your choice (as an integer): ");
        io::stdout().flush().unwrap();

        let mut choice_str = String::new();
        io::stdin().read_line(&mut choice_str).unwrap();
        let choice_result = choice_str.trim().parse::<i32>();

        match choice_result {
            Ok(choice) => match choice {
                1 => {
                    // Pass `teams` to `build_new_coach`
                    let new_coach = build_new_coach(players, teams);
                    coaches.push(new_coach);
                    println!("\nNew coach successfully added!");
                }
                2 => {
                    remove_coach(coaches);
                }
                3 => {
                    println!("\nAll Coaches:");
                    Coach::print_all(coaches);
                }
                4 => {
                    print_coach_team(coaches, players);
                }
                5 => {
                    println!("Returning to the main menu...");
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


/// Displays a submenu for managing pro stats.
fn pro_stats_menu(players: &[ProStats]) {
    loop {
        println!("\nPro Stats Menu:");
        println!("1: Print All");
        println!("2: Print by Name");
        println!("3: Print by Team");
        println!("4: Print Top Scores Per Role");
        println!("5: Return to Main Menu");
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

/// Prompts the user to build a new coach with player inputs.
fn build_new_coach(players: &[ProStats], teams: &[Team]) -> Coach {
    let mut input = String::new();

    // Prompt for coach name.
    print!("Enter Coach Name: ");
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let coach_name = input.trim().to_string();

    // Prompt for each role. Note that we pass the expected role string in lowercase.
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


/// Prompts for a player name for the given role until a valid one is entered.
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
                candidate,
                expected_role
            );
        }
    }
}

fn prompt_for_valid_team(teams: &[Team]) -> String {
    let mut input = String::new();

    loop {
        print!("Enter LCK Team Name: ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let team_name = input.trim();

        // Use the `teamname` getter for validation
        if teams.iter().any(|t| t.teamname().eq_ignore_ascii_case(team_name)) {
            return team_name.to_string();
        } else {
            println!("Invalid team name. Please enter a valid LCK team name.");
        }
    }
}


/// Removes a coach by name.
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

/// Prints a coach's team by name.
fn print_coach_team(coaches: &[Coach], players: &[ProStats]) {
    print!("Enter the coach's name to view their team: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let target_coach = input.trim();

    Coach::print_team_by_coach(coaches, players, target_coach);
}

/// Prints player stats by name.
fn print_pro_stat_by_name(players: &[ProStats]) {
    print!("Enter the player's name: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let player_name = input.trim();

    ProStats::print_by_name(players, player_name);
}

/// Prints player stats by team.
fn print_pro_stat_by_team(players: &[ProStats]) {
    print!("Enter the team's name: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let team_name = input.trim();

    ProStats::print_by_team(players, team_name);
}
