use std::io;
use super::{interact, Player};

pub fn show_stats(others: &Vec<&mut Player>) {
    for other in others {
        interact::print_width(&format!(
            "\n{} {}:
            \nNobles:\t{}\
            \nSoldiers:\t{}\
            \nClergy:\t{}\
            \nMerchants:\t{}\
            \nSerfs:\t{}\
            \nLand:\t{}\
            \nTreasury:\t{}",
            other.title.as_str(), other.name, other.nobles, other.soldiers,
            other.clergy, other.merchants, other.serfs, other.land,
            other.treasury
        ));
    }

    interact::print_width("\n\n(Press [ENTER] to continue.)");
    io::stdin().read_line(&mut String::new()).unwrap();
}
