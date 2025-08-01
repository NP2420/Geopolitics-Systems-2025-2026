use crate::army::*;
use crate::troop::*;

use colored::*;

/*
 * Note: Attacking doesn't mean offense, it means the unit attacking the other unit. Same thing with defending.
 * 
 * Attacking: The unit that attcks the defending unit
 * Defending: The unit defending from the attacking unit (consider renaming)
 * 
 * Offense: The person invading the other
 * Defense: The person on defense from the invasion
 * 
 * In this version, all disabling ratios happen before each specific run
 */

/*
 * Constants:
 * - Runs describes the number of battles/iterations it goes through to get a result. More runs favours a larger force to a certain extent.
 */

const RUNS: u8 = 3;

/*
 * Testing contains terminal stats and information
 * 
 * Returns: Output from calc
 */

pub fn test_calc(offense: &mut Army, defense: &mut Army, offense_boost: u32, defense_boost: u32) -> bool {
    let off_pre_str = offense.val();
    let def_pre_str = defense.val();

    println!("{}", "BEFORE".bold().blue());
    println!("Offense Army: {}Value: {}\n\nDefense Army: {}Value: {}\n", offense, off_pre_str.to_string().yellow(), defense, def_pre_str.to_string().yellow());

    let result = calc(offense, defense, offense_boost, defense_boost);

    let off_pos_str = offense.val();
    let def_pos_str = defense.val();

    println!("{}", "AFTER".bold().blue());
    println!("Offense Army: {}Value: {}\n\nDefense Army: {}Value: {}\n", offense, off_pos_str.to_string().yellow(), defense, def_pos_str.to_string().yellow());
    println!("Offense wins: {} || Defense wins: {}", result, !result);
    println!("Offense: {} || Defense: {}", offense.capture_strength(), defense.capture_strength());

    println!("{}", "\nSTATS".bold().blue());
    
    println!("Offense Value Difference Pre {}", (off_pre_str - def_pre_str).to_string().green());

    println!("Offense Value Lost: {}", (off_pre_str - off_pos_str).to_string().red());
    println!("Defense Value Lost: {}", (def_pre_str - def_pos_str).to_string().red());

    println!("Offense Value Difference Post {}", (off_pos_str - def_pos_str).to_string().green());

    result
}

/*
 * This will cause the offense and defense armies to loose the correct amount of troops
 * 
 * Returns: True if offense wins; False if defense wins
 */

pub fn calc(off: &mut Army, def: &mut Army, off_boost: u32, def_boost: u32) -> bool {
    //Runs (Battle)
    for _ in 0..RUNS {
        //The disabling ratios go before fighting
        fight_disables(off, def, off_boost, def_boost);
        fight_disables(def, off, def_boost, off_boost);

        let def_copy = def.clone();

        //The normal ratios now fight
        fight_armies(off, def, off_boost, def_boost);
        fight_armies(&def_copy, off, def_boost, off_boost);

        //Remove units that have died from hashmap
        off.remove_dead();
        def.remove_dead();
    }

    //Round Armies
    off.round_army();
    def.round_army();

    //Outcome (Winner of territory)
    off.capture_strength() > def.capture_strength()
}

/*
 * All disabling interactions fight
 */

pub fn fight_disables(att_army: &Army, def_army: &mut Army, att_boost: u32, def_boost: u32) {
    for (att_troop, att_amt) in att_army.units.iter() {
        
        let def_tot = allocation_tot(att_troop, def_army);

        for (def_troop, def_amt) in def_army.units.iter_mut() {
            let matchup = att_troop.get_matchup(*def_troop);

            if matchup != Ratio::Disable {
                continue;
            }

            let ratio = Ratio::Arrow.ratio(); //Disable ratio uses Arrow ratio for allocation purposes
            let allocated: f32 = ((def_troop.get_default().value() as f32 + def_troop.def_add() as f32) * ratio * ratio) / def_tot;

            fight_troop(*att_troop, *att_amt, att_boost, allocated, *def_troop, def_amt, def_boost);
        }
    }
}

/*
 * All none-disabling interactions fight
 */

pub fn fight_armies(att_army: &Army, def_army: &mut Army, att_boost: u32, def_boost: u32) {
    for (att_troop, att_amt) in att_army.units.iter() {

        let def_tot = allocation_tot(att_troop, def_army);

        for (def_troop, def_amt) in def_army.units.iter_mut() {
            let matchup = att_troop.get_matchup(*def_troop);

            if matchup == Ratio::Disable {
                continue;
            }

            let ratio = matchup.ratio();
            let allocated = if def_tot == 0.0 { 0.0 } else { ( (def_troop.get_default().value() as f32 + def_troop.def_add() as f32) * ratio * ratio) / def_tot };

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

        def_tot += (def_troop.get_default().value() as f32 + def_troop.def_add() as f32) * def_amt.count * ratio * ratio;
    }

    def_tot
}

/*
 * One unit fights a troop
 */

pub fn fight_troop(att_troop: Troop, att_amt: TroopAmt, att_boost: u32, allocated: f32, def_troop: Troop, def_amt: &mut TroopAmt, def_boost: u32) {
    let att_mult = att_boost as f32 * 0.01 + 1.0;
    let def_mult = def_boost as f32 * 0.01 + 1.0;

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

    (*def_amt).count = if normalize <= 0.0 { 0.0 } else { def_value / normalize };
}

/*
 * This enum defines the $ / power when deciding winner of battle
 */

#[derive(Clone, Copy, Debug)]
pub enum CaptureStrength {
    Amazing,
    Good,
    Bad,
    Terrible,
    Unable
}

impl CaptureStrength {
    pub const fn ratio(self) -> f32 {
        match self {
            CaptureStrength::Amazing => 0.75,
            CaptureStrength::Good => 1.0,
            CaptureStrength::Bad => 1.5,
            CaptureStrength::Terrible => 2.0,
            CaptureStrength::Unable => 0.0
        }
    }
}