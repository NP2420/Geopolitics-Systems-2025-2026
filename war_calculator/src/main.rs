mod troop;
mod army;
mod war_calc;
mod territory_calc;
mod territory;
mod calc_tools;

use crate::{army::Army, territory::Territory, territory_calc::*, troop::*, war_calc::*};

pub fn main() {
    let off_boost = 0;
    let def_boost = 0;

    let mut army = Army::new(String::from("NP"));
    army.add(Troop::Default(DefaultTroop::Infantry), 100.0);
    army.add(Troop::Custom{
        troop_type: DefaultTroop::Infantry, 
        offensive_add: 80000,
        defensive_add: 80000,
        stealth_add: 5}, 100.0);

    let territory = Territory::new(String::from("Canada"), 2, 5);
    test_territory_capture(&mut army, &territory);

    let mut army2 = Army::new(String::from("NP2"));
    army2.add(Troop::Default(DefaultTroop::Infantry), 10.0);
    army2.add(Troop::Default(DefaultTroop::SpecialForces), 10.0);
    army2.add(Troop::Default(DefaultTroop::Tank), 10.0);
    army2.add(Troop::Default(DefaultTroop::Artillery), 10.0);
    army2.add(Troop::Default(DefaultTroop::FlakCannon), 10.0);
    army2.add(Troop::Default(DefaultTroop::Fighter), 10.0);
    army2.add(Troop::Default(DefaultTroop::Bomber), 10.0);
    army2.add(Troop::Default(DefaultTroop::Battleship), 10.0);
    army2.add(Troop::Default(DefaultTroop::Cruiser), 10.0);
    army2.add(Troop::Default(DefaultTroop::Destroyer), 10.0);
    army2.add(Troop::Default(DefaultTroop::Submarine), 10.0);

    test_calc(&mut army, &mut army2, off_boost, def_boost);
}