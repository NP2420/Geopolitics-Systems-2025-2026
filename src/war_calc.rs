use crate::army::*;
use crate::troop::*;
use crate::ratio::*;

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

    let result = calc(&mut off_clone, &mut def_clone, offense_boost, defense_boost);
    
    println!("Offense wins: {} || Defense wins: {}", result, !result);
    println!("Offense Army: {}\nDefense Army: {}", off_clone, def_clone);

    result
}

/*
 * This will cause the offense and defense armies to loose the correct amount of troops.
 * 
 * Returns: True if offense wins; False if defense wins
 */

pub fn calc(off: &mut Army, def: &mut Army, off_boost: i16, def_boost: i16) -> bool {
    //disable stuff
    for _ in 0..RUNS {
        let def_copy = def.clone();
        fight_armies(off, def, off_boost, def_boost);
        fight_armies(&def_copy, off, off_boost, def_boost);
    }
    //Outcome
    true
}

pub fn val(troop: Troop, count: u32) -> f32 {
    count as f32 * (troop.get_default().value() as f32 + troop.def_add() as f32)
}

pub fn fight_armies(att_army: &Army, def_army: &mut Army, att_boost: i16, def_boost: i16) {
    for (att_troop, att_count) in att_army.units.iter() {
        let mut def_tot = 0.0;

        for (def_troop, def_count) in def_army.units.iter() {
            let matchup = att_troop.get_matchup(*def_troop);
            let ratio = if matchup == Ratio::Disable { Ratio::Arrow.ratio() } else { matchup.ratio() };

            def_tot += val(*def_troop, *def_count) * ratio * ratio;
        }

        for (def_troop, def_count) in def_army.units.iter_mut() {
            let matchup = att_troop.get_matchup(*def_troop);
            let ratio = if matchup == Ratio::Disable { Ratio::Arrow.ratio() } else { matchup.ratio() };
            
            let allocated = (val(*def_troop, *def_count) * ratio * ratio) / def_tot;

            fight_troops(*att_troop, *att_count, att_boost, allocated, *def_troop, def_count, def_boost);
        }
    }
}

pub fn fight_troops(att_troop: Troop, att_count: u32, att_boost: i16, allocated: f32, def_troop: Troop, def_count: &mut u32, def_boost: i16) {
    let att_mult = (att_boost as f32) * 0.1 + 1.0;
    let def_mult = (def_boost as f32) * 0.1 + 1.0;

    let att_base = att_troop.get_default().value() as f32 + att_troop.off_add() as f32;
    let def_base = def_troop.get_default().value() as f32 + def_troop.def_add() as f32;

    let mut att_value = att_base * att_count as f32 * att_mult * allocated / RUNS as f32;
    let mut def_value = def_base * (*def_count as f32) * def_mult;

    if att_value < 0.0 { att_value = 0.0; }

    def_value -= att_value * att_troop.get_matchup(def_troop).ratio();

    if def_value < 0.0 { def_value = 0.0; }

    let normalize = def_base * def_mult;

    if normalize <= 0.0 {
        *def_count = 0;
    } else {
        def_value /= normalize;
        *def_count = def_value.round() as u32;
    }
}