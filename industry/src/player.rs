use crate::resources::Resource;
use crate::industry::Stocks;

use std::{collections::HashMap};

const INITIAL_REVENUE: f64 = 10_000_000.0;
const INITIAL_MORALE: u8 = 50;
const INITIAL_RESOURCES: u32 = 0;
const INITIAL_MONEY: f64 = 0.0;
const INITIAL_POPULATION: u32 = 0;
const INITIAL_PERCEPTION: u16 = 0;

struct Player {
    name: String,
    player_stats: PlayerStats,
    industry_stats: IndustryStats
}

impl Player {
    fn new(name: String) -> Player {
        Player {
            name,
            player_stats: PlayerStats::new(),
            industry_stats: IndustryStats::new()
        }
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
            .collect::<HashMap<Resource, u32>>();

        PlayerStats {
            money: INITIAL_MONEY,
            resources,
            morale: INITIAL_MORALE,
            population: INITIAL_POPULATION,
            perception: INITIAL_PERCEPTION
        }
    }
}

struct IndustryStats {
    revenue: f64,
    investments: Stocks,
    foundations: Stocks
}

impl IndustryStats {
    fn new() -> IndustryStats {
        IndustryStats {
            revenue: INITIAL_REVENUE,
            investments: Stocks::new(0.0),
            foundations: Stocks::new(0.0)
        }
    }
}