use crate::{industry::GlobalStocks,resources::Resource};
use crate::industry::Stocks;

use std::{collections::HashMap,fmt};

const INITIAL_REVENUE: f64 = 10_000_000.0;
const INITIAL_MORALE: u8 = 50;
const INITIAL_RESOURCES: u32 = 0;
const INITIAL_MONEY: f64 = 0.0;
const INITIAL_POPULATION: u32 = 0;
const INITIAL_PERCEPTION: u16 = 0;

pub struct Player {
    name: String,
    player_stats: PlayerStats,
    pub industry_stats: IndustryStats
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            player_stats: PlayerStats::new(),
            industry_stats: IndustryStats::new()
        }
    }

    pub fn end_industry(&mut self, industry: &GlobalStocks) { //still need to figure out how to use global stocks  
        self.industry_stats.end_industry(industry);
        self.player_stats.end_industry(&self.industry_stats);
    }
}

struct PlayerStats {
    money: f64, //Or unsigned
    resources: HashMap<Resource, u32>,
    morale: u8, //Or float
    population: u32,
    perception: u16
}

impl PlayerStats {
    fn new() -> PlayerStats {
        let resources = Resource::ALL
            .iter()
            .map(|name| (*name, INITIAL_RESOURCES))
            .collect();

        PlayerStats {
            money: INITIAL_MONEY,
            resources,
            morale: INITIAL_MORALE,
            population: INITIAL_POPULATION,
            perception: INITIAL_PERCEPTION
        }
    }

    fn end_industry(&mut self, industry_stats: &IndustryStats) {
        self.money += industry_stats.revenue;
        //get resources
    }
}

pub struct IndustryStats {
    revenue: f64,
    investments: Stocks,
    foundations: Stocks
}

impl IndustryStats {
    pub fn new() -> IndustryStats {
        IndustryStats {
            revenue: INITIAL_REVENUE,
            investments: Stocks::new(0.0),
            foundations: Stocks::new(0.0)
        }
    }

    fn end_industry(&mut self, industry: &GlobalStocks) {
        let uninvested_revenue = self.revenue - self.investments.sum() as f64;

        for (name, value) in self.investments.iter_mut() {
            let old_value = *value;

            *value *= industry.get_percent(name);

            //Foundation Growth

            *self.foundations.get_mut(name) += *value - old_value;
        }

        self.revenue = uninvested_revenue + self.investments.sum() as f64;
    }

    // pub fn TEMP_distribute(&mut self, vec: Vec<f32>) { //remove later
    //     for ((_, value), new_amount) in self.investments.stocks.iter_mut().zip(vec.into_iter()) {
    //         *value = new_amount * self.revenue as f32;
    //     }
    // }
}

impl fmt::Display for IndustryStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.investments)
    }
}