use num::clamp;
use num_format::{Locale::en, ToFormattedString};
use rand::{Rng, rngs::ThreadRng};
use std::{
    cmp::{max, min, Ordering},
    error::Error,
    io,
};
use super::{
    interact, Player,
    super::{enums::Justice, stats::show_stats},
};

impl<'a> Player<'a> {
    fn add_revenue(&mut self) {
        self.treasury += self.justice_revenue + (
            self.customs_duty_revenue + self.income_tax_revenue + self.sales_tax_revenue
        ) as i32;

        // Penalize deficit spending.
        if self.treasury < 0 {
            self.treasury = (self.treasury as f32 * 1.5) as i32;
        }

        // Will a title make the creditors happy (for now)?
        if self.treasury < (-10_000 * self.title_num as i32) {
            self.bankrupt = true;
        }
    }

    pub fn adjust_tax(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            interact::print_width(&format!(
                "\n{} {}", self.title.as_str(), self.name));

            self.generate_income();

            interact::print_width(&format!(
                "\n  1: Customs Duty (currently {}%)\
                 \n  2: Sales Tax (currently {}%)\
                 \n  3: Wealth Tax (currently {}%)\
                 \n  4: Justice (currently {})\
                 \n\nEnter tax number for changes, or 0 to continue:",
                self.customs_duty,
                self.sales_tax,
                self.income_tax,
                self.justice.as_str()
            ));

            match interact::read_number() {
                Ok(0) => break,
                Ok(1) => {
                    interact::print_width("New customs duty (0 to 100):");

                    match interact::read_number() {
                        Err(_) => interact::print_width("I do not understand your request."),
                        Ok(x) => self.customs_duty = min(x, 100) as u8,
                    }
                }
                Ok(2) => {
                    interact::print_width("New sales tax (0 to 50):");

                    match interact::read_number() {
                        Err(_) => interact::print_width("I do not understand your request."),
                        Ok(x) => self.sales_tax = min(x, 50) as u8,
                    }
                }
                Ok(3) => {
                    interact::print_width("New income tax (0 to 25):");

                    match interact::read_number() {
                        Err(_) => interact::print_width("I do not understand your request."),
                        Ok(x) => self.income_tax = min(x, 25) as u8,
                    }
                }
                Ok(4) => {
                    interact::print_width("\nJustice:\
                        \n  1: Very fair\
                        \n  2: Moderate\
                        \n  3: Harsh\
                        \n  4: Outrageous");

                    match interact::read_number() {
                        Ok(1) => self.justice = Justice::VeryFair,
                        Ok(2) => self.justice = Justice::Moderate,
                        Ok(3) => self.justice = Justice::Harsh,
                        Ok(4) => self.justice = Justice::Outrageous,
                        _ => interact::print_width("I do not understand your request."),
                    }
                }
                _ => interact::print_width("I do not understand your request."),
            }
        }

        self.add_revenue();

        if self.bankrupt {
            self.seize_assets()?;
        }

        Ok(())
    }

    fn generate_income(&mut self) {
        self.justice_revenue = (self.justice.as_int() as i32 * 300 - 500) * self.title_num as i32;

        let y = max(
            1,
            150 - self.sales_tax - self.customs_duty - self.income_tax,
        ) as f32 / 100.0;

        self.customs_duty_revenue =
            ((self.nobles * 180 + self.clergy * 75 + self.merchants * 20) as f32 * y) as u32
                + self.public_works;
        self.customs_duty_revenue =
            (self.customs_duty as f32 / 100.0 * self.customs_duty_revenue as f32) as u32;
        self.sales_tax_revenue = (((self.nobles * 50 + self.merchants * 25) as f32
            + (self.public_works / 10) as f32
                * y
                * (5 - self.justice.as_int()) as f32
                * self.sales_tax as f32)
            / 200.0) as u32;
        self.income_tax_revenue = ((self.nobles as f32 * 250.0
            + self.public_works as f32 / 5.0
            + 10.0
                * self.justice.as_int() as f32
                * self.nobles as f32
                * y
                * self.income_tax as f32)
            / 100.0) as u32;
        let revenues = (
            self.customs_duty_revenue + self.sales_tax_revenue + self.income_tax_revenue
        ) as i32 + self.justice_revenue;

        interact::print_width(&format!(
            "\nState revenues {} gold florins.\
             \nFrom Customs Duty:\t{}\
             \nFrom Sales Tax:\t\t{}\
             \nFrom Income tax:\t{}\
             \nJustice Revenue:\t{} ({})",
            revenues.to_formatted_string(&en),
            self.customs_duty_revenue.to_formatted_string(&en),
            self.sales_tax_revenue.to_formatted_string(&en),
            self.income_tax_revenue.to_formatted_string(&en),
            self.justice_revenue.to_formatted_string(&en),
            self.justice.as_str(),
        ));
    }

    pub fn new_land_and_grain_prices(&mut self, rng: &mut ThreadRng) {

        let y = max((self.serfs as i32 - self.mills as i32) * 500, 0) as u32;
        let x = min(y, self.land);
        let x = min(x, self.grain_reserve * 2);

        let h = x as i32 * (self.harvest as f32 + rng.gen_range(-0.5, 0.5)) as i32;
        self.grain_reserve += h as u32;
        self.grain_demand = self.nobles * 100
            + self.cathedral * 40
            + self.merchants * 30
            + self.soldiers * 10
            + self.serfs * 5;

        let y = match h.cmp(&1) {
            Ordering::Less => 2.0,
            _ => clamp(self.grain_demand as f32 / h as f32, 0.8, 2.0),
        };
        self.land_price = ((3 * self.harvest + rng.gen_range(10, 16)) as f32 / 10.0 * y).max(1.0);
        self.grain_price =
            ((6 - self.harvest) * 3 + rng.gen_range(0, 5) + rng.gen_range(0, 5)) as f32 * 4.0 * y;
    }

    fn seize_assets(&mut self) -> Result<(), Box<dyn Error>> {

        self.marketplaces = 0;
        self.palace = 0;
        self.cathedral = 0;
        self.mills = 0;
        self.land = 6_000;
        self.public_works = 100;
        self.treasury = 100;
        self.bankrupt = false;

        interact::print_width(&format!(
            "\n\n{} {} is bankrupt.
            \nCreditors have seized much of your assets.
            \n\n(Press [ENTER] to continue.)",
            self.title.as_str(),
            self.name
        ));
        io::stdin().read_line(&mut String::new()).unwrap();

        Ok(())
    }

    pub fn sell_land(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            interact::print_width("How many hectares of land do you wish to sell?");

            match interact::read_number() {
                Ok(x) if x > self.land - 5_000 => interact::print_width("You can't sell that much."),
                Ok(x) => {
                    self.land -= x;
                    self.treasury += (x as f32 * self.land_price) as i32;
                    break;
                }
                Err(_) => interact::print_width("I do not understand your request."),
            }
        }
        Ok(())
    }

    pub fn state_purchases(&mut self, others: &Vec<&mut Player>, rng: &mut ThreadRng) -> Result<(), Box<dyn Error>> {
        loop {
            interact::print_width(&format!(
                "\n\n{} {}
                \nState purchases.\
                \n  1: Marketplace ({})\t\t\t\t1,000 florins\
                \n  2: Woollen mill ({})\t\t\t\t2,000 florins\
                \n  3: Palace (partial, {})\t\t\t3,000 florins\
                \n  4: Cathedral (partial, {})\t\t\t5,000 florins\
                \n  5: Equip one platoon of serfs as soldiers\t500florins\
                \n  6: Compare standings\
                \n\nYou have {} gold florins.\
                \n(Or q to continue)",
                self.title.as_str(),
                self.name,
                self.marketplaces,
                self.mills,
                self.palace,
                self.cathedral,
                self.treasury
            ));

            let mut entry = String::new();
            io::stdin().read_line(&mut entry)?;
            match entry.trim().to_lowercase().as_str() {
                "q" => break,
                "1" => self.buy_market(),
                "2" => self.buy_mill(),
                "3" => self.buy_palace(rng),
                "4" => self.buy_cathedral(rng),
                "5" => self.buy_soldiers(),
                "6" => show_stats(others),
                _ => interact::print_width("\nEntry not recognised."),
            }
        }
        Ok(())
    }

    pub fn steres_to_florins(&self, grain: u32) -> u32 {
        (grain as f32 * self.grain_price / 1_000.0) as u32
    }
}
