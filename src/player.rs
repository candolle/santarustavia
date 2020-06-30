mod buy;
mod grain;
mod treasury;
mod war;

use num::clamp;
use num_format::{Locale::en, ToFormattedString};
use rand::{Rng, rngs::ThreadRng, seq::SliceRandom};
use std::{
    collections::VecDeque,
    cmp::{max, min, Ordering},
    error::Error,
    io,
};
use super::{
    enums::{
        Cities, Justice,
        titles::{FemaleTitles, MaleTitles, Titles::{self, Female, Male}}
    },
    interact,
};

// fn draw_map() {
//     unimplemented!();
// }

#[derive(Clone, Debug, PartialEq)]
pub struct Player<'a> {
    bankrupt: bool,
    cathedral: u32,
    city: &'a Cities,
    pub clergy: u32,
    customs_duty: u8,
    customs_duty_revenue: u32,
    pub dead: bool,
    dead_serfs: u32,
    difficulty: u8,
    fleeing_serfs: u32,
    grain_demand: u32,
    grain_price: f32,
    grain_reserve: u32,
    harvest: u8,
    income_tax: u8,
    income_tax_revenue: u32,
    invade_me: bool,
    justice: Justice,
    justice_revenue: i32,
    pub land: u32,
    land_price: f32,
    male: bool,
    marketplaces: u32,
    market_revenue: u32,
    pub merchants: u32,
    mill_revenue: u32,
    mills: u32,
    pub name: String,
    new_serfs: u32,
    pub nobles: u32,
    old_title_num: u32,
    palace: u32,
    public_works: u32,
    rats: f32,
    rats_ate: u32,
    sales_tax: u8,
    sales_tax_revenue: u32,
    pub serfs: u32,
    soldier_pay: u32,
    pub soldiers: u32,
    pub title: Titles,
    title_num: u32,
    transplanted_serfs: u32,
    pub treasury: i32,
    which_player: &'a Cities,
    pub won: bool,
    year: u32,
    year_of_death: u32,
}

impl<'a> Player<'a> {
    pub fn new(year: u32, city: &'a Cities, level: u8, name: String, male: bool, rng: &mut ThreadRng) -> Player<'a> {
        Player {
            bankrupt: false,
            cathedral: 0,
            city,
            clergy: 5,
            customs_duty: 25,
            customs_duty_revenue: 0,
            dead: false,
            dead_serfs: 0,
            difficulty: level,
            fleeing_serfs: 0,
            grain_demand: 0,
            grain_price: 25.0,
            grain_reserve: 5_000,
            harvest: 3,
            income_tax: 5,
            income_tax_revenue: 0,
            invade_me: false,
            justice: Justice::Moderate,
            justice_revenue: 0,
            land: 10_000,
            land_price: 10.0,
            male,
            marketplaces: 0,
            market_revenue: 0,
            merchants: 25,
            mill_revenue: 0,
            mills: 0,
            name,
            new_serfs: 0,
            nobles: 4,
            old_title_num: 1,
            palace: 0,
            public_works: 100,
            rats: 0.0,
            rats_ate: 0,
            sales_tax: 10,
            sales_tax_revenue: 0,
            serfs: 2_000,
            soldier_pay: 0,
            soldiers: 25,
            title: if city == &Cities::Monterana {
                Male(MaleTitles::Baron)
            } else if male {
                Male(MaleTitles::Sir)
            } else {
                Female(FemaleTitles::Lady)
            },
            title_num: 1,
            transplanted_serfs: 0,
            treasury: 1_000,
            which_player: city,
            won: false,
            year,
            year_of_death: year + 20 + rng.gen_range(0, 35),
        }
    }

    fn check_new_title(&mut self) {
        // Tally up our success so far...
        let total = min(10, self.marketplaces)
            + min(10, self.palace)
            + min(10, self.cathedral)
            + min(10, self.mills)
            + max(0, min(10, self.treasury / 5_000)) as u32
            + min(10, self.land / 6_000)
            + min(10, self.merchants / 50)
            + min(10, self.nobles / 5)
            + min(10, self.soldiers / 50)
            + min(10, self.clergy / 10)
            + min(10, self.serfs / 2_000)
            + min(10, self.public_works / 5);

        self.title_num = clamp(
            (total / self.difficulty as u32) - self.justice.as_int() as u32,
            0,
            7,
        );

        if self.title_num > self.old_title_num {
            self.old_title_num = self.title_num;
            self.promote();
            interact::print_width(&format!(
                "\nGood news! {} has achieved the rank of {}.",
                self.name,
                self.title.as_str()
            ));
        } else {
            self.title_num = self.old_title_num;
        }
    }

    fn obituary(&self, rng: &mut ThreadRng) {
        let choices = [
            "after attempting to fly whilst intoxicated.",
            "after being attacked by robbers whilst travelling.",
            "after being attacked by wolves whilst out hunting.",
            "after being eaten by a grue.",
            "after being stabbed by a lowly peasant angered by tax.",
            "after consuming a little too much of the cook's secret sauce.",
            "after contracting tuberculosis.",
            "after eating poisonous berries.",
            "after falling from a high wall during a drunken singing contest.",
            "at the hands of a jealous love rival.",
            "by a spurned maiden from the slums.",
            "due to an act of God.",
            "following an audience with a mad subject with murder in his heart.",
            "from a lifetime love of cake.",
            "from a surfeit of lampreys.",
            "from eating too many game pies.",
            "from hypothermia.",
            "from the enthusiastic mistakes of the royal doctor.",
            "from the plague.",
            "from the shock of unexpected betrayal by a member of the court.",
            "from the stress of turbulent times.",
            "from waist cancer.",
            "in a freak clown accident.",
            "in a smallpox epidemic.",
            "of food poisoning.",
            "of pneumonia after a cold winter in a drafty castle.",
            "of typhoid after drinking contaminated water.",
        ];
        let reason = match self.year.cmp(&1450) {
            Ordering::Greater => "of old age after a long reign.",
            _ => choices.choose(rng).unwrap(),
        };

        interact::print_width(&format!(
            "\n\nVery sad news. {} {} has just died {}\n\n(Press [ENTER] to \
                continue.)",
            self.title.as_str(), self.name, reason
        ));
        io::stdin().read_line(&mut String::new()).unwrap();
    }

    fn promote(&mut self) {
        self.title = match self.title {
            Male(MaleTitles::Sir) => Male(MaleTitles::Baron),
            Male(MaleTitles::Baron) => Male(MaleTitles::Count),
            Male(MaleTitles::Count) => Male(MaleTitles::Marquis),
            Male(MaleTitles::Marquis) => Male(MaleTitles::Duke),
            Male(MaleTitles::Duke) => Male(MaleTitles::GrandDuke),
            Male(MaleTitles::GrandDuke) => Male(MaleTitles::Prince),
            Male(MaleTitles::Prince) => Male(MaleTitles::HRHKing),
            Male(MaleTitles::HRHKing) => Male(MaleTitles::HRHKing),
            Female(FemaleTitles::Lady) => Female(FemaleTitles::Baroness),
            Female(FemaleTitles::Baroness) => Female(FemaleTitles::Countess),
            Female(FemaleTitles::Countess) => Female(FemaleTitles::Marquise),
            Female(FemaleTitles::Marquise) => Female(FemaleTitles::Duchess),
            Female(FemaleTitles::Duchess) => Female(FemaleTitles::GrandDuchess),
            Female(FemaleTitles::GrandDuchess) => Female(FemaleTitles::Princess),
            Female(FemaleTitles::Princess) => Female(FemaleTitles::HRHQueen),
            Female(FemaleTitles::HRHQueen) => Female(FemaleTitles::HRHQueen),
        };
        self.won = self.title == Male(MaleTitles::HRHKing)
            || self.title == Female(FemaleTitles::HRHQueen);
    }

    fn serfs_decomposing(&mut self, scale: f32, rng: &mut ThreadRng) {
        let absc = scale as u32;
        let ord = scale - absc as f32;

        self.dead_serfs = (
            ((rng.gen_range(0.0, scale) + ord) * self.serfs as f32) / 100.0
        ) as u32;
        self.serfs -= self.dead_serfs;

        interact::print_width(&format!(
            "\n{} serfs die this year.",
            self.dead_serfs.to_formatted_string(&en)
        ));
    }

    fn serfs_procreating(&mut self, scale: f32, rng: &mut ThreadRng) {
        self.new_serfs = (
            (rng.gen_range(0.0, scale) * self.serfs as f32) / 100.0
        ) as u32;
        self.serfs += self.new_serfs;

        interact::print_width(&format!(
            "\n{} serfs born this year.",
            self.new_serfs.to_formatted_string(&en)
        ));
    }

    pub fn take_turn(&mut self, others: &mut VecDeque<Player>, baron: &mut Player, rng: &mut ThreadRng) -> Result<(), Box<dyn Error>> {
        interact::print_width(&format!("{}'s turn", self.name));
        self.harvest_grain(rng);
        self.new_land_and_grain_prices(rng);
        self.trade_grain()?;
        self.release_grain(rng)?;

        let mut others_post_invasion: Vec<&mut Player> = vec![];
        if self.invade_me {
            let mut attacked = false;
            for invader in others {
                if invader.soldiers > (self.soldiers as f32 * 2.4) as u32 {
                    self.attacked(&mut *invader, rng);
                    attacked = true;
                }
                others_post_invasion.push(&mut *invader);
            }
            if !attacked {
                self.attacked(baron, rng);
            }
        } else {
            for invader in others {
                others_post_invasion.push(&mut *invader);
            }
        }

        self.adjust_tax()?;
        // draw_map();
        self.state_purchases(&others_post_invasion, rng)?;
        self.check_new_title();

        self.year += 1;
        if self.year == self.year_of_death {
            self.dead = true;
            self.obituary(rng);
        }
        self.won = self.title_num >= 7;
        Ok(())
    }
}
