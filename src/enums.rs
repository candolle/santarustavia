pub mod titles;

#[derive(Debug, PartialEq)]
pub enum Cities {
    SantaRustavia,
    Fiumaccio,
    Torricella,
    Molinetto,
    Fontanile,
    Romanga,
    Monterana,
}

impl Cities {
    pub fn as_str(&self) -> &str {
        match self {
            Cities::SantaRustavia => "Santa Rustavia",
            Cities::Fiumaccio => "Fiumaccio",
            Cities::Torricella => "Torricella",
            Cities::Molinetto => "Molinetto",
            Cities::Fontanile => "Fontanile",
            Cities::Romanga => "Romanga",
            Cities::Monterana => "Monterana",
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Justice {
    VeryFair,
    Moderate,
    Harsh,
    Outrageous,
}

impl Justice {
    pub fn as_int(&self) -> u8 {
        match self {
            Justice::VeryFair => 1,
            Justice::Moderate => 2,
            Justice::Harsh => 3,
            Justice::Outrageous => 4,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Justice::VeryFair => "Very Fair",
            Justice::Moderate => "Moderate",
            Justice::Harsh => "Harsh",
            Justice::Outrageous => "Outrageous",
        }
    }
}
