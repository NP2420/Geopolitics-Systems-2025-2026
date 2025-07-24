use crate::troop::Troop;

use std::{collections::HashMap,fmt};

#[derive(Clone)]
pub struct Army {
    name: String,
    pub units: HashMap<Troop, u32> //Troop type, Count
}

impl Army {
    pub fn new(name: String) -> Self { //This prob also should have a default name instead of an insert (maybe, maybe not)
        Army {
            name,
            units: HashMap::new(),
        }
    }

    pub fn add(&mut self, troop: Troop, count: u32) {
        *self.units.entry(troop).or_insert(0) += count;
    }

    pub fn subtract(&mut self, troop: Troop, count: u32) {
        let cur = self.units.get_mut(&troop);
        if cur.is_none() {
            return;
        }

        let cur_val = cur.unwrap();
        if *cur_val <= count {
            self.units.remove(&troop);
        }   else {
            *cur_val -= count;
        }
    }

    // pub fn remove_all(&mut self) {
    //     self.units.drain();
    // }

    // pub fn edit_name(&mut self, new_name: String) {
    //     self.name = new_name; //I removed pub from name and added this because later there should prob be a check to make sure there is no repeated names.
    // }
}

impl fmt::Display for Army {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Army: {}", self.name)?;
        for (troop, count) in &self.units {
            writeln!(f, "  {}: {}", troop, count)?;
        }
        Ok(())
    }
}