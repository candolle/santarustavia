use num_format::{Locale::en, ToFormattedString};
use rand::{Rng, rngs::ThreadRng};
use std::cmp::min;
use super::{interact, Player, super::enums::Cities};

impl<'a> Player<'a> {
    pub fn attacked(&mut self, attacker: &mut Player, rng: &mut ThreadRng) {
        let mut land_taken = match attacker.city {
            // If the attacker is the Baron (the computer player)
            &Cities::Monterana => rng.gen_range(1_000, 10_000),
            // If it's another human player
            _ => (attacker.soldiers * 1000) - (attacker.land / 3),
        };

        if land_taken > self.land - 5000 {
            land_taken = (self.land - 5000) / 2;
        }

        attacker.land += land_taken;
        self.land -= land_taken;

        interact::print_width(&format!(
            "\n{} {} of {} invades and seizes {} hectares of land!",
            attacker.title.as_str(),
            attacker.name,
            attacker.city.as_str(),
            land_taken.to_formatted_string(&en)
        ));

        let dead_soldiers = min(
            rng.gen_range(0, 40),
            self.soldiers - 15
        );
        self.soldiers -= dead_soldiers;

        interact::print_width(&format!(
            "\n{} {} loses {} soldiers in battle.",
            self.title.as_str(),
            self.name,
            dead_soldiers.to_formatted_string(&en)
        ));
    }
}
