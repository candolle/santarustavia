use num_format::{Locale::en, ToFormattedString};
use std::{cmp::max, error::Error, io};
use rand::{Rng, rngs::ThreadRng};
use super::{interact, Player, super::enums::Justice};

impl<'a> Player<'a> {
    fn buy_grain(&mut self) -> Result<(), Box<dyn Error>> {
        let how_much: u32;
        loop {
            interact::print_width("How much grain do you want to buy (0 \
                to specify a total, negative to cancel purchase)?");

            match interact::read_number() {
                Err(_) => {
                    interact::print_width("Purchase cancelled.");
                    how_much = 0;
                    break
                },
                Ok(0) => {
                    interact::print_width(&format!(
                        "How much total grain do you wish? (You have {} steres)",
                        self.grain_reserve.to_formatted_string(&en)
                    ));
                    match interact::read_number() {
                        Err(_) | Ok(0) => {
                            interact::print_width("Purchase cancelled.");
                            how_much = 0;
                            break
                        },
                        Ok(x) if x <= self.grain_reserve => interact::print_width(
                            "That is less than you already have!"),
                        Ok(x) if self.steres_to_florins(x - self.grain_reserve) as i32 > self.treasury => interact::print_width(
                            "You don't have enough in your treasury."),
                        Ok(x) => {
                            how_much = x;
                            break
                        }
                    }
                },
                Ok(x) if self.steres_to_florins(x) as i32 > self.treasury => interact::print_width(
                    "You don't have enough in your treasury."),
                Ok(x) => {
                    how_much = x;
                    break
                },
            }
        }
        self.treasury -= self.steres_to_florins(how_much) as i32;
        self.grain_reserve += how_much;
        Ok(())
    }

    pub fn harvest_grain(&mut self, rng: &mut ThreadRng) {
        self.harvest = (rng.gen_range(1, 6) + rng.gen_range(1, 6)) / 2;
        // Rats eat up to half
        self.rats = rng.gen_range(0.0, 0.5);
        self.rats_ate = (self.grain_reserve as f32 * self.rats) as u32;
        self.grain_reserve -= self.rats_ate;
    }

    pub fn release_grain(&mut self, rng: &mut ThreadRng) -> Result<(), Box<dyn Error>> {
        let how_much: u32;
        let minimum = self.grain_reserve / 5;
        let maximum = self.grain_reserve - minimum;
        loop {
            interact::print_width(&format!(
                "\nHow much grain will you release for consumption?\
                \n  1: Minimum ({} steres)\
                \n  2: Maximum ({} steres)\
                \nor enter a value:",
                minimum.to_formatted_string(&en), maximum.to_formatted_string(&en)
            ));

            match interact::read_number() {
                Err(_) => interact::print_width("\nNot a valid option."),
                Ok(1) => {
                    how_much = minimum;
                    break
                },
                Ok(2) => {
                    how_much = maximum;
                    break
                },
                Ok(a) if a < minimum => interact::print_width("\nYou must release at least 20% of your reserves."),
                Ok(a) if a > maximum => interact::print_width("\nYou must keep at least 20% of your reserves."),
                Ok(a) => {
                    how_much = a;
                    break
                }
            };
        }

        self.market_revenue = 0;
        self.new_serfs = 0;
        self.soldier_pay = 0;
        self.dead_serfs = 0;
        self.transplanted_serfs = 0;
        self.fleeing_serfs = 0;
        self.invade_me = false;
        self.grain_reserve -= how_much;

        // let mut z = match how_much as f32 / self.grain_demand as f32 - 1.0 {
        //     a if a > 0.0 => a / 2.0,
        //     a => a,
        // };

        // z = match z {
        //     a if a > 0.25 => a / 10.0 + 0.25,
        //     a => a,
        // };

        // let mut zp = match 50.0 - self.customs_duty as f32 - self.sales_tax as f32 - self.income_tax as f32 {
        //     a if a < 0.0 => a * self.justice as f32,
        //     a => a,
        // } / 10.0;

        // zp = match zp {
        //     a if a > 0.0 => a + 3.0 - self.justice as f32,
        //     a => a,
        // };

        // z += zp / 10.0;

        // z = z.min(0.5);

        if how_much < self.grain_demand - 1 {
            let x = (self.grain_demand - how_much) as f32 / self.grain_demand as f32 * 100.0 - 9.0;
            let xp = x.max(0.0);
            // x = clamp(x, 0.0, 65.0);
            self.serfs_procreating(3.0, rng);
            self.serfs_decomposing(xp + 8.0, rng);
        } else {
            self.serfs_procreating(7.0, rng);
            self.serfs_decomposing(3.0, rng);
            if self.customs_duty + self.sales_tax < 35 {
                self.merchants += rng.gen_range(0, 4);
            }

            if self.income_tax < rng.gen_range(0, 28) {
                self.nobles += rng.gen_range(0, 2);
                self.clergy += rng.gen_range(0, 3);
            }

            if how_much > (self.grain_demand as f32 * 1.3) as u32 {
                let mut zp = self.serfs as f32 / 1000.0;
                let mut z = (how_much - self.grain_demand) as f32 / self.grain_demand as f32 * 10.0;
                z *= zp * rng.gen_range(0.0, 25.0);
                z += rng.gen_range(0.0, 40.0);
                self.transplanted_serfs = z as u32;
                self.serfs += self.transplanted_serfs;
                interact::print_width(&format!(
                    "\n{} serfs move to the city.",
                    self.transplanted_serfs.to_formatted_string(&en)
                ));
                zp = z;
                z = (zp * rng.gen::<f32>()).min(50.0);
                self.merchants += z as u32;
                self.nobles += 1;
                self.clergy += 2;
            }
        }

        if self.justice > Justice::Moderate {
            self.justice_revenue = rng.gen_range(
                0,
                self.serfs as i32 / 100 * if self.justice == Justice::Outrageous { 4 } else { 1 }
            );
            self.serfs -= max(0, self.justice_revenue) as u32;
            self.fleeing_serfs = max(0, self.justice_revenue) as u32;
            interact::print_width(&format!(
                "\n{} serfs flee harsh justice.",
                self.fleeing_serfs.to_formatted_string(&en))
            );
        }

        self.market_revenue = self.marketplaces * 75;
        if self.market_revenue > 0 {
            self.treasury += self.market_revenue as i32;
            interact::print_width(&format!(
                "\nYour market earned {} florins.",
                self.market_revenue.to_formatted_string(&en))
            );
        }

        self.mill_revenue = self.mills * rng.gen_range(55, 305);
        if self.mill_revenue > 0 {
            self.treasury += self.mill_revenue as i32;
            interact::print_width(&format!(
                "\nYour woollen mill earned {} florins.",
                self.mill_revenue.to_formatted_string(&en))
            );
        }

        self.soldier_pay = self.soldiers * 3;
        self.treasury -= self.soldier_pay as i32;
        interact::print_width(&format!(
            "\nYou paid your soldiers {} florins.
            \nYou have {} serfs in your city.
            \n\n(Press [ENTER] to continue.)",
            self.soldier_pay.to_formatted_string(&en),
            self.serfs.to_formatted_string(&en)
        ));
        io::stdin().read_line(&mut String::new()).unwrap();

        self.invade_me = self.land / 500 > self.soldiers;
        Ok(())
    }

    fn sell_grain(&mut self) -> Result<(), Box<dyn Error>> {
        let how_much: u32;
        loop {
            interact::print_width("How much grain do you want to sell?");

            match interact::read_number() {
                Ok(x) if x > self.grain_reserve => interact::print_width(
                    "You don't have enough in your granary."),
                Ok(x) => {
                    how_much = x;
                    break
                },
                _ => {
                    interact::print_width("Sale cancelled.");
                    how_much = 0;
                    break
                },
            }
        }
        self.treasury += self.steres_to_florins(how_much) as i32;
        self.grain_reserve -= how_much;
        Ok(())
    }

    pub fn trade_grain(&mut self) -> Result<(), Box<dyn Error>> {
        interact::print_width(&format!(
            "\nYear {}: {} {}\n\
            \nRats ate {}% of your grain reserves. ({} steres)",
            self.year,
            self.title.as_str(), self.name,
            (self.rats * 100.0) as u32, self.rats_ate.to_formatted_string(&en)
        ));
        let text = match self.harvest {
            1 => "Drought. Famine threatens.",
            2 => "Bad weather. Poor harvest.",
            3 => "Normal weather. Average harvest.",
            4 => "Good weather. Fine harvest.",
            5 => "Excellent weather. Great harvest!",
            _ => "Unpredictable weather. Who knows what it means for the harvest?",
        };
        interact::print_width(text);
        loop {
            interact::print_width(&format!(
                "\nGrain Reserve:\t{} steres\
                \nGrain Demand:\t{} steres\
                \nPrice of Grain:\t{} per 1,000 steres\
                \nPrice of Land:\t{} per hectare\
                \nTreasury:\t{} gold florins
                \nYou have {} hectares of land. What would you like to do?\
                \n  1: Buy grain\
                \n  2: Sell grain\
                \n  3: Buy land\
                \n  4: Sell land\
                \n\n(Or q to continue)",
                self.grain_reserve.to_formatted_string(&en),
                self.grain_demand.to_formatted_string(&en),
                format!("{:.*}", 2, self.grain_price),
                format!("{:.*}", 2, self.land_price),
                self.treasury.to_formatted_string(&en),
                self.land.to_formatted_string(&en)
            ));

            let mut entry = String::new();
            io::stdin().read_line(&mut entry)?;
            match entry.trim().to_lowercase().as_str() {
                "q" => break,
                "1" => self.buy_grain()?,
                "2" => self.sell_grain()?,
                "3" => self.buy_land()?,
                "4" => self.sell_land()?,
                _ => interact::print_width("\nEntry not recognised."),
            };

        }
        Ok(())
    }
}
