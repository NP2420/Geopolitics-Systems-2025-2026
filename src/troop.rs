use crate::capture_strength::*;
use crate::ratio::*;

use std::fmt;

/*
 * This file defines the `Troop` enum, which represents different types of troops
 * Each troop has
 * - A monetary value (purchasing)
 * - A strength capability when taking territories
 * - A stealth level to determine whether or not it is seen
 * - A name
 */

 
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Troop {
    Default(DefaultTroop),
    Custom {
        troop_type: DefaultTroop,
        offensive_add: i32,
        defensive_add: i32,
        stealth_add: i32
    } // Troop Type, Offensive Additive, Defensive Additive, Stealth Additive
}

impl fmt::Display for Troop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Troop::Default(t) => write!(f, "{}", t),
            Troop::Custom { 
                troop_type, offensive_add, defensive_add, stealth_add 
            } => write!(f, "{} ({}, {}, {})", troop_type, offensive_add, defensive_add, stealth_add),
        }
    }
}

impl Troop {
    pub fn get_matchup(self, other: Troop) -> Ratio {
        self.get_default().get_ratio(other.get_default())
    }

    pub fn get_default(self) -> DefaultTroop {
        match self {
            Troop::Default(t) => t,
            Troop::Custom { troop_type, .. } => troop_type,
        }
    }

    pub fn off_add(self) -> i32 {
        match self {
            Troop::Default(..) => 0,
            Troop::Custom { offensive_add,.. } => offensive_add,
        }
    }

    pub fn def_add(self) -> i32 {
        match self {
            Troop::Default(..) => 0,
            Troop::Custom { defensive_add,.. } => defensive_add,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum DefaultTroop {
    Infantry,
    SpecialForces,
    Tank,
    Artillery,
    FlakCannon,
    Bomber,
    Fighter,
    Battleship,
    Cruiser,
    Destroyer,
    Submarine,
}

impl fmt::Display for DefaultTroop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            DefaultTroop::Infantry => "Infantry",
            DefaultTroop::SpecialForces => "Special Forces",
            DefaultTroop::Tank => "Tank",
            DefaultTroop::Artillery => "Artillery",
            DefaultTroop::FlakCannon => "Flak Cannon",
            DefaultTroop::Bomber => "Bomber",
            DefaultTroop::Fighter => "Fighter",
            DefaultTroop::Battleship => "Battleship",
            DefaultTroop::Cruiser => "Cruiser",
            DefaultTroop::Destroyer => "Destroyer",
            DefaultTroop::Submarine => "Submarine"
        };
        write!(f, "{}", name)
    }
}

impl DefaultTroop {
    pub fn value(self) -> u32 {
        match self {
            DefaultTroop::Infantry => 80_000,
            DefaultTroop::SpecialForces => 160_000,
            DefaultTroop::Tank => 240_000,
            DefaultTroop::Artillery => 240_000,
            DefaultTroop::FlakCannon => 240_000,
            DefaultTroop::Bomber => 320_000,
            DefaultTroop::Fighter => 320_000,
            DefaultTroop::Battleship => 400_000,
            DefaultTroop::Cruiser => 240_000,
            DefaultTroop::Destroyer => 240_000,
            DefaultTroop::Submarine => 240_000,
        }
    }

    pub fn capture_strength(self) -> f32 {
        let strength = match self {
            DefaultTroop::Infantry => CaptureStrength::Amazing,
            DefaultTroop::SpecialForces => CaptureStrength::Bad,
            DefaultTroop::Tank => CaptureStrength::Good,
            DefaultTroop::Artillery => CaptureStrength::Bad,
            DefaultTroop::FlakCannon => CaptureStrength::Terrible,
            DefaultTroop::Bomber => CaptureStrength::Bad,
            DefaultTroop::Fighter => CaptureStrength::Terrible,
            DefaultTroop::Battleship => CaptureStrength::Bad, //Double check
            DefaultTroop::Cruiser => CaptureStrength::Unable,
            DefaultTroop::Destroyer => CaptureStrength::Unable,
            DefaultTroop::Submarine => CaptureStrength::Unable,
        };

        if strength.ratio() == 0.0 { 0.0 } else { (self.value() as f32) / strength.ratio() }
    }

    pub fn stealth(self) -> i32 {
        match self {
            DefaultTroop::Infantry => 0,
            DefaultTroop::SpecialForces => 1,
            DefaultTroop::Tank => 0,
            DefaultTroop::Artillery => 0,
            DefaultTroop::FlakCannon => 0,
            DefaultTroop::Bomber => 0,
            DefaultTroop::Fighter => 0,
            DefaultTroop::Battleship => 0,
            DefaultTroop::Cruiser => 0,
            DefaultTroop::Destroyer => 0,
            DefaultTroop::Submarine => 1
        }
    }

    pub fn get_ratio(self, def: DefaultTroop) -> Ratio {
        match (self, def) {
            (DefaultTroop::Infantry, DefaultTroop::Infantry) => Ratio::ArrowToSelf,
            (DefaultTroop::Infantry, DefaultTroop::SpecialForces) => Ratio::Unable,
            (DefaultTroop::Infantry, DefaultTroop::Tank) => Ratio::BadColor,
            (DefaultTroop::Infantry, DefaultTroop::Artillery) => Ratio::Arrow,
            (DefaultTroop::Infantry, DefaultTroop::FlakCannon) => Ratio::Arrow,
            (DefaultTroop::Infantry, DefaultTroop::Bomber) => Ratio::BadColor,
            (DefaultTroop::Infantry, DefaultTroop::Fighter) => Ratio::BadColor,
            (DefaultTroop::Infantry, DefaultTroop::Battleship) => Ratio::Unable,
            (DefaultTroop::Infantry, DefaultTroop::Cruiser) => Ratio::Unable,
            (DefaultTroop::Infantry, DefaultTroop::Destroyer) => Ratio::Unable,
            (DefaultTroop::Infantry, DefaultTroop::Submarine) => Ratio::Unable,

            (DefaultTroop::SpecialForces, DefaultTroop::Infantry) => Ratio::Unable,
            (DefaultTroop::SpecialForces, DefaultTroop::SpecialForces) => Ratio::ArrowToSelf,
            (DefaultTroop::SpecialForces, DefaultTroop::Tank) => Ratio::Unable,
            (DefaultTroop::SpecialForces, DefaultTroop::Artillery) => Ratio::Disable,
            (DefaultTroop::SpecialForces, DefaultTroop::FlakCannon) => Ratio::Disable,
            (DefaultTroop::SpecialForces, DefaultTroop::Bomber) => Ratio::Unable,
            (DefaultTroop::SpecialForces, DefaultTroop::Fighter) => Ratio::Unable,
            (DefaultTroop::SpecialForces, DefaultTroop::Battleship) => Ratio::Unable,
            (DefaultTroop::SpecialForces, DefaultTroop::Cruiser) => Ratio::Unable,
            (DefaultTroop::SpecialForces, DefaultTroop::Destroyer) => Ratio::Unable,
            (DefaultTroop::SpecialForces, DefaultTroop::Submarine) => Ratio::Unable,

            (DefaultTroop::Tank, DefaultTroop::Infantry) => Ratio::Arrow,
            (DefaultTroop::Tank, DefaultTroop::SpecialForces) => Ratio::Unable,
            (DefaultTroop::Tank, DefaultTroop::Tank) => Ratio::ArrowToSelf,
            (DefaultTroop::Tank, DefaultTroop::Artillery) => Ratio::BadColor,
            (DefaultTroop::Tank, DefaultTroop::FlakCannon) => Ratio::BadColor,
            (DefaultTroop::Tank, DefaultTroop::Bomber) => Ratio::Unable,
            (DefaultTroop::Tank, DefaultTroop::Fighter) => Ratio::Unable,
            (DefaultTroop::Tank, DefaultTroop::Battleship) => Ratio::Unable,
            (DefaultTroop::Tank, DefaultTroop::Cruiser) => Ratio::Unable,
            (DefaultTroop::Tank, DefaultTroop::Destroyer) => Ratio::Unable,
            (DefaultTroop::Tank, DefaultTroop::Submarine) => Ratio::Unable,

            (DefaultTroop::Artillery, DefaultTroop::Infantry) => Ratio::BadColor,
            (DefaultTroop::Artillery, DefaultTroop::SpecialForces) => Ratio::Unable,
            (DefaultTroop::Artillery, DefaultTroop::Tank) => Ratio::Arrow,
            (DefaultTroop::Artillery, DefaultTroop::Artillery) => Ratio::BadColor,
            (DefaultTroop::Artillery, DefaultTroop::FlakCannon) => Ratio::BadColor,
            (DefaultTroop::Artillery, DefaultTroop::Bomber) => Ratio::Unable,
            (DefaultTroop::Artillery, DefaultTroop::Fighter) => Ratio::Unable,
            (DefaultTroop::Artillery, DefaultTroop::Battleship) => Ratio::Unable,
            (DefaultTroop::Artillery, DefaultTroop::Cruiser) => Ratio::Unable,
            (DefaultTroop::Artillery, DefaultTroop::Destroyer) => Ratio::Unable,
            (DefaultTroop::Artillery, DefaultTroop::Submarine) => Ratio::Unable,

            (DefaultTroop::FlakCannon, DefaultTroop::Infantry) => Ratio::BadColor,
            (DefaultTroop::FlakCannon, DefaultTroop::SpecialForces) => Ratio::Unable,
            (DefaultTroop::FlakCannon, DefaultTroop::Tank) => Ratio::OkColor,
            (DefaultTroop::FlakCannon, DefaultTroop::Artillery) => Ratio::BadColor,
            (DefaultTroop::FlakCannon, DefaultTroop::FlakCannon) => Ratio::BadColor,
            (DefaultTroop::FlakCannon, DefaultTroop::Bomber) => Ratio::Arrow,
            (DefaultTroop::FlakCannon, DefaultTroop::Fighter) => Ratio::Arrow,
            (DefaultTroop::FlakCannon, DefaultTroop::Battleship) => Ratio::Unable,
            (DefaultTroop::FlakCannon, DefaultTroop::Cruiser) => Ratio::Unable,
            (DefaultTroop::FlakCannon, DefaultTroop::Destroyer) => Ratio::Unable,
            (DefaultTroop::FlakCannon, DefaultTroop::Submarine) => Ratio::Unable,

            (DefaultTroop::Bomber, DefaultTroop::Infantry) => Ratio::Arrow,
            (DefaultTroop::Bomber, DefaultTroop::SpecialForces) => Ratio::Unable,
            (DefaultTroop::Bomber, DefaultTroop::Tank) => Ratio::OkColor,
            (DefaultTroop::Bomber, DefaultTroop::Artillery) => Ratio::BadColor,
            (DefaultTroop::Bomber, DefaultTroop::FlakCannon) => Ratio::BadColor,
            (DefaultTroop::Bomber, DefaultTroop::Bomber) => Ratio::BadColor,
            (DefaultTroop::Bomber, DefaultTroop::Fighter) => Ratio::BadColor,
            (DefaultTroop::Bomber, DefaultTroop::Battleship) => Ratio::Arrow,
            (DefaultTroop::Bomber, DefaultTroop::Cruiser) => Ratio::BadColor,
            (DefaultTroop::Bomber, DefaultTroop::Destroyer) => Ratio::BadColor,
            (DefaultTroop::Bomber, DefaultTroop::Submarine) => Ratio::Unable,

            (DefaultTroop::Fighter, DefaultTroop::Infantry) => Ratio::OkColor,
            (DefaultTroop::Fighter, DefaultTroop::SpecialForces) => Ratio::Unable,
            (DefaultTroop::Fighter, DefaultTroop::Tank) => Ratio::BadColor,
            (DefaultTroop::Fighter, DefaultTroop::Artillery) => Ratio::BadColor,
            (DefaultTroop::Fighter, DefaultTroop::FlakCannon) => Ratio::BadColor,
            (DefaultTroop::Fighter, DefaultTroop::Bomber) => Ratio::Arrow,
            (DefaultTroop::Fighter, DefaultTroop::Fighter) => Ratio::ArrowToSelf,
            (DefaultTroop::Fighter, DefaultTroop::Battleship) => Ratio::OkColor,
            (DefaultTroop::Fighter, DefaultTroop::Cruiser) => Ratio::BadColor,
            (DefaultTroop::Fighter, DefaultTroop::Destroyer) => Ratio::BadColor,
            (DefaultTroop::Fighter, DefaultTroop::Submarine) => Ratio::Unable,

            (DefaultTroop::Battleship, DefaultTroop::Infantry) => Ratio::OkColor,
            (DefaultTroop::Battleship, DefaultTroop::SpecialForces) => Ratio::Unable,
            (DefaultTroop::Battleship, DefaultTroop::Tank) => Ratio::OkColor,
            (DefaultTroop::Battleship, DefaultTroop::Artillery) => Ratio::OkColor,
            (DefaultTroop::Battleship, DefaultTroop::FlakCannon) => Ratio::OkColor,
            (DefaultTroop::Battleship, DefaultTroop::Bomber) => Ratio::BadColor,
            (DefaultTroop::Battleship, DefaultTroop::Fighter) => Ratio::BadColor,
            (DefaultTroop::Battleship, DefaultTroop::Battleship) => Ratio::OkColor,
            (DefaultTroop::Battleship, DefaultTroop::Cruiser) => Ratio::Arrow,
            (DefaultTroop::Battleship, DefaultTroop::Destroyer) => Ratio::Arrow,
            (DefaultTroop::Battleship, DefaultTroop::Submarine) => Ratio::Unable,

            (DefaultTroop::Cruiser, DefaultTroop::Infantry) => Ratio::Unable,
            (DefaultTroop::Cruiser, DefaultTroop::SpecialForces) => Ratio::Unable,
            (DefaultTroop::Cruiser, DefaultTroop::Tank) => Ratio::Unable,
            (DefaultTroop::Cruiser, DefaultTroop::Artillery) => Ratio::Unable,
            (DefaultTroop::Cruiser, DefaultTroop::FlakCannon) => Ratio::Unable,
            (DefaultTroop::Cruiser, DefaultTroop::Bomber) => Ratio::Arrow,
            (DefaultTroop::Cruiser, DefaultTroop::Fighter) => Ratio::Arrow,
            (DefaultTroop::Cruiser, DefaultTroop::Battleship) => Ratio::BadColor,
            (DefaultTroop::Cruiser, DefaultTroop::Cruiser) => Ratio::BadColor,
            (DefaultTroop::Cruiser, DefaultTroop::Destroyer) => Ratio::BadColor,
            (DefaultTroop::Cruiser, DefaultTroop::Submarine) => Ratio::Unable,

            (DefaultTroop::Destroyer, DefaultTroop::Infantry) => Ratio::Unable,
            (DefaultTroop::Destroyer, DefaultTroop::SpecialForces) => Ratio::Unable,
            (DefaultTroop::Destroyer, DefaultTroop::Tank) => Ratio::Unable,
            (DefaultTroop::Destroyer, DefaultTroop::Artillery) => Ratio::Unable,
            (DefaultTroop::Destroyer, DefaultTroop::FlakCannon) => Ratio::Unable,
            (DefaultTroop::Destroyer, DefaultTroop::Bomber) => Ratio::Unable,
            (DefaultTroop::Destroyer, DefaultTroop::Fighter) => Ratio::Unable,
            (DefaultTroop::Destroyer, DefaultTroop::Battleship) => Ratio::BadColor,
            (DefaultTroop::Destroyer, DefaultTroop::Cruiser) => Ratio::BadColor,
            (DefaultTroop::Destroyer, DefaultTroop::Destroyer) => Ratio::BadColor,
            (DefaultTroop::Destroyer, DefaultTroop::Submarine) => Ratio::Disable,

            (DefaultTroop::Submarine, DefaultTroop::Infantry) => Ratio::Unable,
            (DefaultTroop::Submarine, DefaultTroop::SpecialForces) => Ratio::Unable,
            (DefaultTroop::Submarine, DefaultTroop::Tank) => Ratio::Unable,
            (DefaultTroop::Submarine, DefaultTroop::Artillery) => Ratio::Unable,
            (DefaultTroop::Submarine, DefaultTroop::FlakCannon) => Ratio::Unable,
            (DefaultTroop::Submarine, DefaultTroop::Bomber) => Ratio::Unable,
            (DefaultTroop::Submarine, DefaultTroop::Fighter) => Ratio::Unable,
            (DefaultTroop::Submarine, DefaultTroop::Battleship) => Ratio::Arrow,
            (DefaultTroop::Submarine, DefaultTroop::Cruiser) => Ratio::BadColor,
            (DefaultTroop::Submarine, DefaultTroop::Destroyer) => Ratio::BadColor,
            (DefaultTroop::Submarine, DefaultTroop::Submarine) => Ratio::ArrowToSelf,
        }
    }
}