mod enums;
mod interact;
mod player;
mod stats;

use enums::Cities;
use player::Player;
use rand::{rngs::ThreadRng, thread_rng};
use std::{collections::VecDeque, error::Error, io};
use textwrap::termwidth;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

fn introduce<'a>(level: u8, city: &'a Cities, rng: &mut ThreadRng) -> Result<Player<'a>, Box<dyn Error>> {
    interact::print_width(&format!(
        "\nWho will be the ruler of {}?",
        city.as_str()
    ));

    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    let name = name.trim().to_string();

    interact::print_width(&format!("Does {} want (m)ale or (f)emale \
        titles?", name));
    let mut entry = String::new();
    io::stdin().read_line(&mut entry)?;
    let male = match entry.trim().to_lowercase().as_str() {
        "m" | "male" => true,
        _ => false,
    };
    
    Ok(Player::new(1400, city, level, name, male, rng))
}

fn play(players: &mut Vec<Player>, rng: &mut ThreadRng) -> Result<(), Box<dyn Error>> {

    let mut baron = Player::new(
        1400, &Cities::Monterana, 4, String::from("Peppone"),
        true, rng
    );

    for round in 1 ..= 60 {
        interact::print_width(&format!(
            "{}\nRound: {}", "-".repeat(termwidth()), round
        ));

        // Sort the players into those that are alive and those that are dead
        let mut round_live = VecDeque::new();
        let mut round_dead = VecDeque::new();
        // drain() renders actual players from the Vec, not &mut players
        for player in players.drain(..) {
            if player.dead {
                round_dead.push_back(player);
            } else {
                round_live.push_back(player);
            }
        }

        if !round_live.is_empty() {
            // Give a turn to each player still in the game
            for _ in 0 .. round_live.len() {
                // Extract the current player
                let mut player = round_live.pop_front().unwrap();
                // Play against the other live players and the baron
                player.take_turn(&mut round_live, &mut baron, rng)?;
                // Return the player to the list
                round_live.push_back(player);
            }
        } else {
            interact::print_width("\nAll players have died. The game has ended.");
            break
        }

        if let Some(x) = round_live.iter().find(|x| x.won) {
            interact::print_width(&format!("\nGame Over. {} {} wins.", x.title.as_str(), x.name));
            break
        };
        for player in round_live {
            players.push(player);
        }
        for player in round_dead {
            players.push(player);
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();

    let city_list = [
        Cities::SantaRustavia,
        Cities::Fiumaccio,
        Cities::Torricella,
        Cities::Molinetto,
        Cities::Fontanile,
        Cities::Romanga,
        Cities::Monterana,
    ];

    interact::instructions()?;

    let players_count = interact::request_player_count()?;

    let level = interact::request_difficulty()?;

    let mut players = Vec::new();
    for city in city_list.iter().take(players_count) {
        let player = introduce(level, city, &mut rng)?;
        players.push(player);
    }

    play(&mut players, &mut rng)?;
    Ok(())
}
