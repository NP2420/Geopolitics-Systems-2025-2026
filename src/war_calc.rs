use crate::army::*;
use crate::troop::*;
use crate::ratio::*;
use crate::troop_amt::*;
use colored::*;

const RUNS: u8 = 3;

/*
 * Note: Attacking doesn't mean offense, it means the unit attacking the other unit. Same thing with defending.
 * 
 * Attacking: The unit that attcks the defending unit
 * Defending: The unit defending from the attacking unit
 * 
 * Offense: The person invading the other
 * Defense: The person on defense from the invasion
 */


/*
 * This is to see what the outcome of a war is without actually losing troops
 */

pub fn test_calc(offense: &Army, defense: &Army, offense_boost: i16, defense_boost: i16) -> bool {
    let mut off_clone = offense.clone();
    let mut def_clone = defense.clone();

    let off_pre_str = army_val(&off_clone);
    let def_pre_str = army_val(&def_clone);

    println!("{}", "BEFORE".bold().blue());
    println!("Offense Army: {}Value: {}\n\nDefense Army: {}Value: {}\n", off_clone, off_pre_str.to_string().yellow(), def_clone, def_pre_str.to_string().yellow());

    let result = calc(&mut off_clone, &mut def_clone, offense_boost, defense_boost);

    let off_pos_str = army_val(&off_clone);
    let def_pos_str = army_val(&def_clone);

    println!("{}", "AFTER".bold().blue());
    println!("Offense Army: {}Value: {}\n\nDefense Army: {}Value: {}\n", off_clone, off_pos_str.to_string().yellow(), def_clone, def_pos_str.to_string().yellow());
    println!("Offense wins: {} || Defense wins: {}", result, !result);
    println!("Offense: {} || Defense: {}", capture_strength_army(&off_clone), capture_strength_army(&def_clone));


    println!("{}", "\nSTATS".bold().blue());
    
    println!("Offense Value Difference Pre {}", (off_pre_str - def_pre_str).to_string().green());

    println!("Offense Value Lost: {}", (off_pre_str - off_pos_str).to_string().red());
    println!("Defense Value Lost: {}", (def_pre_str - def_pos_str).to_string().red());

    println!("Offense Value Difference Post {}", (off_pos_str - def_pos_str).to_string().green());


    result
}

/*
 * This will cause the offense and defense armies to loose the correct amount of troops.
 * 
 * Returns: True if offense wins; False if defense wins
 */

pub fn calc(off: &mut Army, def: &mut Army, off_boost: i16, def_boost: i16) -> bool {
    //Special Forces disable before anything else
    for _ in 0..RUNS {
        let def_copy = def.clone();
        fight_special_forces(off, def, off_boost, def_boost);
        fight_special_forces(&def_copy, off, off_boost, def_boost);
    }

    //Runs (Battle)
    for _ in 0..RUNS {
        let def_copy = def.clone();
        fight_all(off, def, off_boost, def_boost);
        fight_all(&def_copy, off, off_boost, def_boost);
    }

    //Round Armies
    round_army(off);
    round_army(def);

    //Outcome (Winner of territory)
    capture_strength_army(off) > capture_strength_army(def)
}

pub fn army_val(army: &Army) -> f32 {
    let mut sum = 0.0;
    for (troop, amt) in army.units.iter() {
        sum += val(*troop, amt.count);
    }
    sum
}

/* 
 * Value of a troop
 */

pub fn val(troop: Troop, count: f32) -> f32 {
    count as f32 * (troop.get_default().value() as f32 + troop.def_add() as f32)
}

/*
 * Special forces act before other army
 */

pub fn fight_special_forces(att_army: &Army, def_army: &mut Army, att_boost: i16, def_boost: i16) {
    for (att_troop, att_amt) in att_army.units.iter() {
        if att_troop.get_default() == DefaultTroop::SpecialForces {
            fight_army(att_troop, att_amt, def_army, att_boost, def_boost);
        }
    }
}

/*
 * Other army act
 */

pub fn fight_all(att_army: &Army, def_army: &mut Army, att_boost: i16, def_boost: i16) {
    for (att_troop, att_amt) in att_army.units.iter() {
        if att_troop.get_default() != DefaultTroop::SpecialForces {
            fight_army(att_troop, att_amt, def_army, att_boost, def_boost);
        }
    }
}

/*
 * One unit fights the army
 */

pub fn fight_army(att_troop: &Troop, att_amt: &TroopAmt, def_army: &mut Army, att_boost: i16, def_boost: i16) {
    let mut def_tot = 0.0;

    for (def_troop, def_amt) in def_army.units.iter() {
        let matchup = att_troop.get_matchup(*def_troop);
        let ratio = if matchup == Ratio::Disable { Ratio::Arrow.ratio() } else { matchup.ratio() };

        def_tot += val(*def_troop, (*def_amt).count) * ratio * ratio;
    }

    for (def_troop, def_amt) in def_army.units.iter_mut() {
        let matchup = att_troop.get_matchup(*def_troop);
        let ratio = if matchup == Ratio::Disable { Ratio::Arrow.ratio() } else { matchup.ratio() };

        let allocated = (val(*def_troop, (*def_amt).count) * ratio * ratio) / def_tot;

        fight_troop(*att_troop, *att_amt, att_boost, allocated, *def_troop, def_amt, def_boost);
    }
}

/*
 * One unit fights a troop
 */

pub fn fight_troop(att_troop: Troop, att_amt: TroopAmt, att_boost: i16, allocated: f32, def_troop: Troop, def_amt: &mut TroopAmt, def_boost: i16) {
    let att_mult = (att_boost as f32) * 0.1 + 1.0;
    let def_mult = (def_boost as f32) * 0.1 + 1.0;

    let att_base = att_troop.get_default().value() as f32 + att_troop.off_add() as f32;
    let def_base = def_troop.get_default().value() as f32 + def_troop.def_add() as f32;

    let mut att_value = att_base * att_amt.count * att_mult * allocated * (1.0 - att_amt.disabled) / RUNS as f32;
    let mut def_value = def_base * (*def_amt).count * def_mult;

    if att_value < 0.0 { att_value = 0.0; }

    let lost = att_value * att_troop.get_matchup(def_troop).ratio();

    if att_troop.get_matchup(def_troop) == Ratio::Disable {
        def_amt.disable(lost / def_value);
        return;
    }

    def_value -= lost;

    if def_value < 0.0 { def_value = 0.0; }

    let normalize = def_base * def_mult;

    if normalize <= 0.0 {
        (*def_amt).count = 0.0;
    } else {
        (*def_amt).count = def_value / normalize;
    }
}

/*
 * Regular round an army
 */

pub fn round_army(army: &mut Army) {
    for (_, amt) in army.units.iter_mut() {
        amt.count = amt.count.round();
    }
}

/*
 * Return capture_strength numbers for an army
 */

pub fn capture_strength_army(army: &Army) -> f32 {
    let mut sum_strength = 0.0;
    for (troop, amt) in army.units.iter() {
        sum_strength += troop.get_default().capture_strength() * amt.count;
    }
    sum_strength
}