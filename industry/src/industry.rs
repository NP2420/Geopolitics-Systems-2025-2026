use std::{collections::HashMap};
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
enum StockName {
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

    pub fn get_percent(&self, name: StockName) -> f32 {
        let sum: f32 = self.stocks.values().sum();
        if sum == 0.0 {
            return 0.0
        }
        self.stocks.get(&name).expect("Stock not found?") / sum
    }
}
struct GlobalStocks {
    stocks: Stocks,
    nat_growth_midpoint: Stocks,
    nat_growth_range: Stocks,
    global_investment: Stocks
}

impl GlobalStocks {
    pub fn new() -> GlobalStocks {
        GlobalStocks {
            stocks: Stocks::new(INITIAL_STOCK_VAL),
            nat_growth_midpoint: Stocks::new(NAT_INITIAL_MIDPOINT),
            nat_growth_range: Stocks::new(NAT_INITIAL_RANGE),
            global_investment: Stocks::new(0.0)
        }
    }

    pub fn grow(&mut self) {
        //Function to update Global Investments ask dannie how all the players are gonna be formatted etc etc or if its up to me

        let mut rng = rand::thread_rng(); //Idk if theres any benefit to having only one rng thread for all this (nat growth and player growth) but whatever

        self.natural_growth(&mut rng);
        self.player_growth(&mut rng);
    }

    pub fn natural_growth(&mut self, rng: &mut ThreadRng) {
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

    pub fn player_growth(&mut self, rng: &mut ThreadRng) {
        for (name, value) in &mut self.stocks.stocks {
            //Increasing Stock Value
            let invested_percent = self.global_investment.get_percent(*name);

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
}