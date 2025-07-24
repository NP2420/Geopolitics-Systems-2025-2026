/*
 * This enum defines the ratios used when one troop fights another troop.
 * 
 * (Example) An infantry against another infantry has ArrowToSelf ratio.
 */

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Ratio {
    Disable,
    Arrow,
    ArrowToSelf,
    OkColor,
    BadColor,
    Unable
}

impl Ratio {
    pub const fn ratio(&self) -> f32 {
        match self {
            Ratio::Disable => 2.0,
            Ratio::Arrow => 1.0,
            Ratio::ArrowToSelf => 0.75,
            Ratio::OkColor => 0.5,
            Ratio::BadColor => 0.25,
            Ratio::Unable => 0.0,
        }
    }
}