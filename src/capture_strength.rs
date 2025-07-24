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