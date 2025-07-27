use crate::troop::Troop;
use crate::army::Army;

/* 
 * Value of a troop
 */

pub fn val(troop: Troop, count: f32) -> f32 {
    count as f32 * troop.get_default().value() as f32
}

/* 
 * Attacking value of a troop
 */

pub fn off_val(troop: Troop, count: f32) -> f32 {
    count as f32 * (troop.get_default().value() as f32 + troop.off_add() as f32)
}

/* 
 * Defending value of a troop
 */

pub fn def_val(troop: Troop, count: f32) -> f32 {
    count as f32 * (troop.get_default().value() as f32 + troop.def_add() as f32)
}

/* 
 * Value of an army
 */

pub fn army_val(army: &Army) -> f32 {
    let mut sum = 0.0;
    for (troop, amt) in army.units.iter() {
        sum += val(*troop, amt.count);
    }
    sum
}

/* 
 * Offensive value of an army
 */

pub fn army_off_val(army: &Army) -> f32 {
    let mut sum = 0.0;
    for (troop, amt) in army.units.iter() {
        sum += off_val(*troop, amt.count);
    }
    sum
}

/*
 * Remove units that have count of less than 0.0
 */

pub fn remove_dead(army: &mut Army) {
    army.units.retain(|_, amt| amt.count > 0.0);
}

/*
 * Round the amt.count of an army
 */

pub fn round_army(army: &mut Army) {
    for (_, amt) in army.units.iter_mut() {
        amt.count = amt.count.round();
    }
}