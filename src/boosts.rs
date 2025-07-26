/*
 * This enum defines the preset boosts each side may receive during a war
 */

pub enum Boost {
    Def_Capital,
    Def_Megalopolis,
    Off_Surrounding,
    Off_Flanking,
}

impl Boost {
    pub fn is_def(&self) -> bool {
        matches!(self, 
            Boost::Def_Capital | 
            Boost::Def_Megalopolis
        )
    }

    pub fn num(&self) -> f32 {
        match self {
            Boost::Def_Capital | Boost::Off_Surrounding => 5.0,
            Boost::Def_Megalopolis | Boost::Off_Flanking => 2.5,
        }
    }
}
