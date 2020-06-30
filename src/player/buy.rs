use rand::{Rng, rngs::ThreadRng};
use std::error::Error;
use super::{interact, Player};

impl<'a> Player<'a> {
    pub fn buy_cathedral(&mut self, rng: &mut ThreadRng) {
        self.cathedral += 1;
        self.clergy += rng.gen_range(0, 6);
        self.treasury -= 5_000;
        self.public_works += 100;
    }

    pub fn buy_land(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            interact::print_width("How many hectares of land do you wish to purchase?");

            match interact::read_number() {
                Ok(x) => {
                    self.treasury -= (x as f32 * self.land_price) as i32;
                    self.land += x;
                    break;
                },
                _ => interact::print_width("I do not understand your request."),
            }
        }
        Ok(())
    }

    pub fn buy_market(&mut self) {
        self.marketplaces += 1;
        self.merchants += 5;
        self.treasury -= 1_000;
        self.public_works += 100;
    }

    pub fn buy_mill(&mut self) {
        self.mills += 1;
        self.treasury -= 2_000;
        self.public_works += 25;
    }

    pub fn buy_palace(&mut self, rng: &mut ThreadRng) {
        self.palace += 1;
        self.nobles += rng.gen_range(0, 2);
        self.treasury -= 3_000;
        self.public_works += 50;
    }

    pub fn buy_soldiers(&mut self) {
        self.soldiers += 20;
        self.serfs -= 20;
        self.treasury -= 500;
    }
}
