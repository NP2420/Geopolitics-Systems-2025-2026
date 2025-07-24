mod troop;
mod ratio;
mod army;
mod war_calc;
mod capture_strength;

use crate::{army::Army, troop::*, war_calc::*};

pub fn main() {
    let off_boost = 0;
    let def_boost = 0;

    let mut army = Army::new(String::from("NP"));
    army.add(Troop::Default(DefaultTroop::Infantry), 8);

    let mut army2 = Army::new(String::from("NP2"));
    army2.add(Troop::Default(DefaultTroop::Artillery), 5);
    army2.add(Troop::Default(DefaultTroop::FlakCannon), 5);


    calc(&mut army, &mut army2, off_boost, def_boost);

    println!("{}", army);
    println!("{}", army2);
}

pub fn main2() {
    let mut army = Army::new(String::from("NP"));
    army.add(Troop::Default(DefaultTroop::Infantry), 5);
    army.add(Troop::Default(DefaultTroop::SpecialForces), 5);
    army.add(Troop::Default(DefaultTroop::Tank), 5);
    army.add(Troop::Default(DefaultTroop::Fighter), 5);
    army.add(Troop::Default(DefaultTroop::FlakCannon), 5);
    army.add(Troop::Default(DefaultTroop::Infantry), 5);
    army.add(Troop::Default(DefaultTroop::FlakCannon), 5);

    // army.add(Troop::Custom{troop_type: DefaultTroop::FlakCannon, offensive_mult: 2, defensive_mult: 3, stealth_add: 4}, 5);
    // army.add(Troop::Custom{troop_type: DefaultTroop::FlakCannon, offensive_mult: 0, defensive_mult: -1, stealth_add: 8}, 6);
    // army.add(Troop::Custom{troop_type: DefaultTroop::Tank, offensive_mult: 0, defensive_mult: -1, stealth_add: 8}, 6);



    let mut army2 = Army::new(String::from("NP2"));
    army2.add(Troop::Default(DefaultTroop::Infantry), 5);
    army2.add(Troop::Default(DefaultTroop::SpecialForces), 5);
    army2.add(Troop::Default(DefaultTroop::Tank), 5);
    army2.add(Troop::Default(DefaultTroop::Fighter), 5);
    army2.add(Troop::Default(DefaultTroop::FlakCannon), 5);
    army2.add(Troop::Default(DefaultTroop::Infantry), 5);
    army2.add(Troop::Default(DefaultTroop::FlakCannon), 5);

    // army2.add(Troop::Custom{troop_type: DefaultTroop::FlakCannon, offensive_mult: 2, defensive_mult: 3, stealth_add: 4}, 5);
    // army2.add(Troop::Custom{troop_type: DefaultTroop::FlakCannon, offensive_mult: 0, defensive_mult: -1, stealth_add: 8}, 6);
    // army2.add(Troop::Custom{troop_type: DefaultTroop::Tank, offensive_mult: 0, defensive_mult: -1, stealth_add: 8}, 6);

    println!("{}", army);
    println!("{}", army2);

}