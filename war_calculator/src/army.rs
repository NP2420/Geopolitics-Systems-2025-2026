use crate::troop::{Troop, TroopAmt};

use std::{collections::HashMap,fmt};

#[derive(Clone)]
pub struct Army {
    name: String,
    pub units: HashMap<Troop, TroopAmt> //Troop type, Count
}

impl Army {
    pub fn new(name: String) -> Self { //Fix naming (It should be automated name base on player/country name)
        Army {
            name,
            units: HashMap::new(),
        }
    }

    pub fn add(&mut self, troop: Troop, count: f32) {
        self.units.entry(troop).or_insert(TroopAmt::new(0.0)).count += count;
    }

    pub fn remove_dead(&mut self) {
        self.units.retain(|_, amt| amt.count > 0.0);
    }

    pub fn round_army(&mut self) {
        for (_, amt) in self.units.iter_mut() {
            amt.count = amt.count.round();
        }
    }

    pub fn val(&self) -> f32 {
        let mut sum = 0.0;
        for (troop, amt) in self.units.iter() {
            sum += troop.val(amt.count);
        }
        sum
    }

    pub fn off_val(&self) -> f32 {
        let mut sum = 0.0;
        for (troop, amt) in self.units.iter() {
            sum += troop.off_val(amt.count);
        }
        sum
    }

    // Likely needed in future additions

    // pub fn subtract(&mut self, troop: Troop, count: f32) {
    //     if let Some(cur_val) = self.units.get_mut(&troop) {
    //         if cur_val.count <= count {
    //             self.units.remove(&troop);
    //         } else {
    //             cur_val.count -= count;
    //         }
    //     }
    // }

    // pub fn remove_all(&mut self) {
    //     self.units.drain();
    // }

    // pub fn edit_name(&mut self, new_name: String) {
    //     self.name = new_name; //I removed pub from name and added this because later there should prob be a check to make sure there is no repeated names.
    // }
}

impl fmt::Display for Army {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.name)?;
        for (troop, amt) in &self.units {
            writeln!(f, "  {}: {}", troop, amt.count)?;
        }
        Ok(())
    }
}