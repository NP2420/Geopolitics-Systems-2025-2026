use crate::army::Army; 
use crate::territory::Territory;
use crate::calc_tools::*;

use colored::*;

const COST_PER_POP: u32 = 1_350_000;
const COST_PER_TIER: u32 = 5_500_000;
const FAILURE_LOST_PERCENT: f32 = 0.15; //On fail, lose capture cost * this percent in army value

pub fn test_territory_capture(army: &mut Army, territory: &Territory) -> bool {

    let army_pre = army_val(army);

    println!("\n{}", "BEFORE".bold().blue());
    println!("Army: {}Value: {}", army, army_pre.to_string().yellow());

    let capture_cost = (territory.tier * COST_PER_TIER + territory.population * COST_PER_POP) as f32;

    let result = territory_capture(army, territory);
    println!("\nCapture Cost: {}\nResult: {}", capture_cost.to_string().red().bold(), result.to_string().red().bold());

    let army_pos = army_val(army);

    println!("\n{}", "AFTER".bold().blue());
    println!("Army: {}Value: {}\n", army, army_pos.to_string().yellow());

    result
}

pub fn territory_capture(army: &mut Army, territory: &Territory) -> bool {
    let capture_cost = (territory.tier * COST_PER_TIER + territory.population * COST_PER_POP) as f32;
    let army_val = army_off_val(army);

    if army_val <= 0.0 {
        return false;
    }

    if capture_cost >= army_val { //Failed Capture
        let percent = capture_cost * FAILURE_LOST_PERCENT / army_val;
        percent_loss(army, percent);

        return false;
    }
    else { //Successful Capture
        let percent = capture_cost / army_val;
        percent_loss(army, percent);

        return true;
    }
}

pub fn percent_loss(army: &mut Army, percent: f32) {
    let clamped = percent.clamp(0.0, 1.0);
    for (_, amt) in army.units.iter_mut() {
        amt.count *= 1.0 - clamped;
    }

    remove_dead(army);
    round_army(army);
}