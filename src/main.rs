mod troop;
mod ratio;
mod army;
mod capture_strength;
mod troop_amt;
mod war_calc;
mod boosts;

use crate::{army::Army, troop::*, war_calc::*};

pub fn main() {
    let off_boost = 5.0;
    let def_boost = 0.0;

    let mut army = Army::new(String::from("NP"));
    army.add(Troop::Default(DefaultTroop::Infantry), 100.0);

    let mut army2 = Army::new(String::from("NP2"));
    army2.add(Troop::Default(DefaultTroop::Infantry), 100.0);

    test_calc(&mut army, &mut army2, off_boost, def_boost);
}