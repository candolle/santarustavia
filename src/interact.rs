use std::{error::Error, io};
use super::{interact, VERSION};
use textwrap::{fill, termwidth};

/// Print to the width of the terminal.
pub fn print_width(text: &str) {
    println!("{}", fill(text, termwidth()));
}

/// Read a non-negative number from the terminal. Non-digit characters will be
/// stripped out, so for example "1,200" is a valid input, in case the player
/// matches the format of numbers that are printed. If the non-digit-stripped
/// input is not a valid non-negative number, Hammurabi will complain and the
/// player will be given the chance to try again.
pub fn read_number() -> Result<u32, Box<dyn Error>> {
    let mut entry = String::new();
    io::stdin().read_line(&mut entry)?;
    entry.retain(|c| c.is_digit(10));
    Ok(entry.parse::<u32>()?)
}

/// Print the instructions
pub fn instructions() -> Result<(), Box<dyn Error>> {
    print_width(&format!("\n    SANTA RUSTAVIA (v{})\n\
        \nDo you wish to read the instructions? (Y/N)", VERSION.unwrap_or("")));

    let mut entry = String::new();
    io::stdin().read_line(&mut entry)?;
    if entry.trim().to_lowercase().as_str() == "y" {
        print_width("\n\nYou are the ruler of a 15th century Italian \
            city-state. If you rule well, you will receive higher titles. The \
            first player to become king or queen wins. Life expectancy then \
            was brief, so you may not live long enough to win.");
        /* The map described is not implemented yet. */
        // print_width("The computer will draw a map of your state. The \
        //     size of the area in the wall grows as you buy more land. The size \
        //     of the guard tower in the upper left corner shows the adequacy of \
        //     your defenses. If it shrinks, equip more soldiers! If the horse \
        //     and plowman is touching the top of the wall, all your land is in \
        //     production. Otherwise you need more serfs, who will migrate to \
        //     your state if you distribute more grain than the minimum demand.");
        print_width("Buy land to increase your ability to feed your \
            population. Ensure your defenses are adequate by equipping more \
            soldiers. Try to get all your land in production. To increase land \
            in production you will need more serfs, who will migrate to \
            your state if you distribute more grain than the minimum demand.");
        print_width("If you distribute less grain, some of your people \
            will starve, and you will have a high death rate. High taxes raise \
            money, but slow down economic growth.\n");
    }

    Ok(())
}

pub fn request_player_count() -> Result<usize, Box<dyn Error>> {
    interact::print_width("\nHow many people want to play (1 to 6)?");
    loop {
        let input = interact::read_number()?;
        match input {
            1 ..= 6 => return Ok(input as usize),
            _ => interact::print_width("\nOnly 1 to 6 players are possible."),
        }
    }
}

pub fn request_difficulty() -> Result<u8, Box<dyn Error>> {
    interact::print_width("\nWhat will be the difficulty of this game?\
        \n  1: Apprentice\
        \n  2: Journeyman\
        \n  3: Master\
        \n  4: Grand Master\
        \n\nChoose:");

    loop {
        let input = interact::read_number()?;
        match input {
            1 ..= 4 => return Ok(input as u8),
            _ => interact::print_width("\nOnly levels 1 to 4 are possible."),
        }
    }
}
