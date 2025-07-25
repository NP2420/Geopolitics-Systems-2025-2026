use crate::army::*;
use crate::troop::*;
use crate::ratio::*;
use crate::troop_amt::*;
use colored::*;

/*
 * ============================================ VERSION TWO (2) =====================================================
 * In this version, all disabling ratios happen before each specific run
 * 
 * Notes:
 * N/A
 */


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
 * Constants:
 * Runs describes the number of battles/iterations it goes through to get a result. More runs favours a larger force to a certain extent.
 */

const RUNS: u8 = 3;

/*
 * This is to see what the outcome and stats of a war is without actually losing troops
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
    
    //Runs (Battle)
    for _ in 0..RUNS {
        //The disabling ratios go before fighting
        fight_disables(off, def, off_boost, def_boost);
        fight_disables(def, off, off_boost, def_boost);

        let def_copy = def.clone();

        //The normal ratios now fight
        fight_armies(off, def, off_boost, def_boost);
        fight_armies(&def_copy, off, off_boost, def_boost);

        //Remove units that have died from hashmap
        remove_dead(off);
        remove_dead(def);
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

pub fn fight_disables(att_army: &Army, def_army: &mut Army, att_boost: i16, def_boost: i16) {
    for (att_troop, att_amt) in att_army.units.iter() {
        
        let def_tot = allocation_tot(att_troop, def_army);

        for (def_troop, def_amt) in def_army.units.iter_mut() {
            let matchup = att_troop.get_matchup(*def_troop);

            if matchup != Ratio::Disable {
                continue;
            }

            let ratio = Ratio::Arrow.ratio(); //Disable ratio uses Arrow ratio for allocation purposes
            let allocated: f32 = (val(*def_troop, (*def_amt).count) * ratio * ratio) / def_tot;

            fight_troop(*att_troop, *att_amt, att_boost, allocated, *def_troop, def_amt, def_boost);
        }
    }
}

pub fn fight_armies(att_army: &Army, def_army: &mut Army, att_boost: i16, def_boost: i16) {
    for (att_troop, att_amt) in att_army.units.iter() {

        let def_tot = allocation_tot(att_troop, def_army);

        for (def_troop, def_amt) in def_army.units.iter_mut() {
            let matchup = att_troop.get_matchup(*def_troop);

            if matchup == Ratio::Disable {
                continue;
            }

            let ratio = matchup.ratio();
            let allocated = (val(*def_troop, (*def_amt).count) * ratio * ratio) / def_tot;

            fight_troop(*att_troop, *att_amt, att_boost, allocated, *def_troop, def_amt, def_boost);
        }
    }
}

/*
 * This takes a troop and adds up the total value of the enemy army for that troop (using the ratios squared)
 */

pub fn allocation_tot(att_troop: &Troop, def_army: &Army) -> f32 {
    let mut def_tot = 0.0;

    for (def_troop, def_amt) in def_army.units.iter() {
        let matchup = att_troop.get_matchup(*def_troop);
        let ratio = if matchup == Ratio::Disable { Ratio::Arrow.ratio() } else { matchup.ratio() };

        def_tot += val(*def_troop, (*def_amt).count) * ratio * ratio;
    }

    def_tot
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

    let matchup = att_troop.get_matchup(def_troop);
    let lost = att_value * matchup.ratio();

    if matchup == Ratio::Disable {
        def_amt.disabled = lost / def_value;
        return;
    }

    def_value -= lost;

    if def_value < 0.0 { def_value = 0.0; }

    let normalize = def_base * def_mult;

    if normalize <= 0.0 { //maybe
        (*def_amt).count = 0.0;
    } else {
        (*def_amt).count = def_value / normalize;
    }
}

/*
 * Remove units that have count of 0
 */

pub fn remove_dead(army: &mut Army) {
    let mut remove = Vec::new();

    for (troop, amt) in army.units.iter() {
        if amt.count == 0.0 {
            remove.push(*troop);
        }
    }

    for troop in remove {
        army.units.remove(&troop);
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