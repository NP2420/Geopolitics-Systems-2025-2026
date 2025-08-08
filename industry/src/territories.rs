use bitflags::bitflags;

use crate::{resources::*, player::*};

enum Region {
    Asia,
    Europe,
    Africa,
    NorthAmerica,
    MiddleEast,
    SouthEastAsiaOceania,
    Siberia,
    SouthAmerica,
}

impl Region {
    pub fn buff(&self) -> Option<Resource> { //Now i recognize this is something to instead pull from db but I'm leaving this here for now and will change it later
        match self { 
            Region::Asia => Some(Resource::Food),
            Region::Europe => Some(Resource::Metal),
            Region::Africa => Some(Resource::Metal),
            Region::NorthAmerica => Some(Resource::Food),
            Region::MiddleEast => Some(Resource::Oil),
            Region::SouthEastAsiaOceania => Some(Resource::Oil),
            Region::Siberia => None,
            Region::SouthAmerica => None
        }
    }

    pub fn debuff(&self) -> Option<Resource> { //Now i recognize this is something to instead pull from db but I'm leaving this here for now and will change it later
        match self { 
            Region::Asia => Some(Resource::Oil),
            Region::Europe => Some(Resource::Food),
            Region::Africa => Some(Resource::Food),
            Region::NorthAmerica => Some(Resource::Oil),
            Region::MiddleEast => Some(Resource::Metal),
            Region::SouthEastAsiaOceania => Some(Resource::Metal),
            Region::Siberia => None,
            Region::SouthAmerica => None
        }
    }
}

enum TerritoryType { //I think this is technically supposed to be a struct...
    Normal,
    Megalopolis,
    Capital,
    TradeCenter
}

struct TerritoryStats {
    name: String,
    tier: u8,
    resource: Resource,
    population: u8,
    region: Region,    
}

impl TerritoryStats {
    pub fn new(name: String, tier: u8, resource: Resource, population: u8, region: Region) -> TerritoryStats {
        TerritoryStats { 
            name,
            tier,
            resource,
            population,
            region,
        }
    }
}

pub struct Territory {
    territory_stats: TerritoryStats,
    owner: Option<Player>,
    flags: bool, //Add in
    factory_level: u8,
    factory_percentage: f32, //Percentage usable (iirc)
    new_population: f32 //still on same scale as previous population
}

bitflags::bitflags! {
    pub struct TerritoryFlags: u64 {
        const NO_MOVEMENT           = 0b0000000001;
        const POISONED              = 0b0000000010;
        const LOWERED_PROD_1        = 0b0000000100;
        const LOWERED_PROD_2        = 0b0000001000;
        const INCREASED_PROD_1      = 0b0000010000;
        const INCREASED_PROD_2      = 0b0000100000;
        const FORTIFICATION_1       = 0b0001000000;
        const FORTIFICATION_2       = 0b0010000000;
        const WEAKENING_1           = 0b0100000000;
        const WEAKENING_2           = 0b1000000000;
    }
}
