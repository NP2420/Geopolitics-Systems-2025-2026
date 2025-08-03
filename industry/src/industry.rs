use std::{collections::HashMap,fmt};
use rand::{Rng, rngs::ThreadRng};

//Constants that should be removed later probably
const INITIAL_STOCK_VAL: f32 = 50_000.0;

const GLOBAL_INVESTMENT_LOWER: f32 = 0.14;
const GLOBAL_INVESTMENT_UPPER: f32 = 0.20;

const NAT_INITIAL_MIDPOINT: f32 = 3_000.0;
const NAT_INITIAL_RANGE: f32 = 4_000.0;

const NAT_MIDPOINT_LOWER: f32 = 1_000.0;
const NAT_MIDPOINT_MIDDLE: f32 = 0.0;
const NAT_MIDPOINT_UPPER: f32 = -1_000.0;

const PLAYER_RANGE: f32 = 3000.0;

const PLAYER_LOWER_MDPT: f32 = 5500.0;
const PLAYER_MIDDLE_MDPT: f32 = -500.0;
const PLAYER_UPPER_MDPT: f32 = 2500.0;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum StockName {
    Health,
    Entertainment,
    Defense,
    Technology,
    Business,
    Manufacturing
}

impl StockName {
    pub const ALL: [StockName; 6] = [
        StockName::Health,
        StockName::Entertainment,
        StockName::Defense,
        StockName::Technology,
        StockName::Business,
        StockName::Manufacturing 
    ];
}

impl fmt::Display for StockName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Health => "Health",
            Self::Entertainment => "Entertainment",
            Self::Defense => "Defense",
            Self::Technology => "Technology",
            Self::Business => "Business",
            Self::Manufacturing => "Manufacturing",
        };
        write!(f, "{}", name)
    }
}

pub struct Stocks {
    pub stocks: HashMap<StockName, f32>
}

impl Stocks {
    pub fn new(value: f32) -> Stocks {
        let stocks = StockName::ALL
            .iter()
            .map(|name| (*name, value))
            .collect::<HashMap<StockName, f32>>();
        Stocks { stocks }
    }

    pub fn sum(&self) -> f32 {
        self.stocks.values().sum()
    }

    fn get_percent(&self, name: StockName) -> f32 {
        let sum = self.sum();
        if sum == 0.0 {
            return 0.0
        }
        self.stocks.get(&name).expect("Stock not found?") / sum
    }
}

impl fmt::Display for Stocks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (name, value) in &self.stocks {
            writeln!(f, "  {}: {}", name, value)?;
        }
        Ok(())
    }
}

pub struct GlobalStocks {
    stocks: Stocks,
    prev_stocks: Stocks,
    nat_growth_midpoint: Stocks,
    nat_growth_range: Stocks,
    global_investment: Stocks
}

impl GlobalStocks {
    pub fn new() -> GlobalStocks {
        GlobalStocks {
            stocks: Stocks::new(INITIAL_STOCK_VAL),
            prev_stocks: Stocks::new(INITIAL_STOCK_VAL),
            nat_growth_midpoint: Stocks::new(NAT_INITIAL_MIDPOINT),
            nat_growth_range: Stocks::new(NAT_INITIAL_RANGE),
            global_investment: Stocks::new(0.0)
        }
    }

    pub fn end_industry(&mut self) {
        //Transfer stocks to prev_stocks
        self.transfer_stocks();

        //Update investments
        //To do

        //Grow Stocks
        let mut rng = rand::thread_rng(); //Idk if theres any benefit to having only one rng thread for all this (nat growth and player growth) but whatever

        self.natural_growth(&mut rng);
        self.player_growth(&mut rng);
    }

    fn transfer_stocks(&mut self) {
        for (name, value) in &self.stocks.stocks {
            self.prev_stocks.stocks.insert(*name, *value);
        }
    }

    fn natural_growth(&mut self, rng: &mut ThreadRng) {
        for (name, value) in &mut self.stocks.stocks {
            //Increasing Stock Value
            let midpoint = self.nat_growth_midpoint.stocks.get_mut(name).expect("Missing midpoint?");
            let range = self.nat_growth_range.stocks.get_mut(name).expect("Missing range?");

            let upper_bound = *midpoint + *range / 2.0;
            let lower_bound = *midpoint - *range / 2.0;

            let growth = rng.gen_range(lower_bound..=upper_bound);

            *value += growth;

            //Updating Range
            let range_change = (upper_bound - *midpoint) / 2.0 - (growth - *midpoint).abs();
            *range += range_change;
            *range = if *range < 0.0 { 0.0 } else { *range };

            //Updating Midpoint
            let invested_percent = self.global_investment.get_percent(*name);
            // let invested_percent = 0.17;

            let midpoint_change = if invested_percent < GLOBAL_INVESTMENT_LOWER {
                NAT_MIDPOINT_LOWER
            } else if invested_percent <= GLOBAL_INVESTMENT_UPPER{
                NAT_MIDPOINT_MIDDLE
            } else {
                NAT_MIDPOINT_UPPER
            };
            
            *midpoint += midpoint_change;
        }
    }

    fn player_growth(&mut self, rng: &mut ThreadRng) {
        for (name, value) in &mut self.stocks.stocks {
            //Increasing Stock Value
            let invested_percent = self.global_investment.get_percent(*name);
            // let invested_percent = 0.22;

            let midpoint = if invested_percent < GLOBAL_INVESTMENT_LOWER {
                PLAYER_LOWER_MDPT
            } else if invested_percent <= GLOBAL_INVESTMENT_UPPER{
                PLAYER_MIDDLE_MDPT
            } else {
                PLAYER_UPPER_MDPT
            };

            let upper_bound = midpoint + PLAYER_RANGE / 2.0;
            let lower_bound = midpoint - PLAYER_RANGE / 2.0;

            let growth = rng.gen_range(lower_bound..=upper_bound);
            *value += growth;
        }
    }

    pub fn get_percent(&self, name: &StockName) -> f32 {
        self.stocks.stocks.get(name).expect("Stock not found?") / self.prev_stocks.stocks.get(name).expect("Stock not found?")
    }
}