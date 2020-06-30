#[derive(Clone, Debug, PartialEq)]
pub enum Titles {
    Male(MaleTitles),
    Female(FemaleTitles),
}

impl Titles {
    pub fn as_str(&self) -> &str {
        match self {
            Titles::Male(x) => x.as_str(),
            Titles::Female(x) => x.as_str(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum FemaleTitles {
    Lady, // 0
    Baroness, // 1
    Countess, // 2
    Marquise, // 3
    Duchess, // 4
    GrandDuchess, // 5
    Princess, // 6
    HRHQueen, // 7
}

impl FemaleTitles {
    pub fn as_str(&self) -> &str {
        match self {
            FemaleTitles::Lady => "Lady",
            FemaleTitles::Baroness => "Baroness",
            FemaleTitles::Countess => "Countess",
            FemaleTitles::Marquise => "Marquise",
            FemaleTitles::Duchess => "Duchess",
            FemaleTitles::GrandDuchess => "Grand Duchess",
            FemaleTitles::Princess => "Princess",
            FemaleTitles::HRHQueen => "* H.R.H. Queen",
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum MaleTitles {
    Sir, // 0
    Baron, // 1
    Count, // 2
    Marquis, // 3
    Duke, // 4
    GrandDuke, // 5
    Prince, // 6
    HRHKing, // 7
}

impl MaleTitles {
    pub fn as_str(&self) -> &str {
        match self {
            MaleTitles::Sir => "Sir",
            MaleTitles::Baron => "Baron",
            MaleTitles::Count => "Count",
            MaleTitles::Marquis => "Marquis",
            MaleTitles::Duke => "Duke",
            MaleTitles::GrandDuke => "Grand Duke",
            MaleTitles::Prince => "Prince",
            MaleTitles::HRHKing => "* H.R.H. King",
        }
    }
}
