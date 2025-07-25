#[derive(Clone, Copy)]
pub struct TroopAmt {
    pub count: f32,
    pub disabled: f32, //Percent of count that is disabled (unusable) 0-1
}

impl TroopAmt {
    pub fn new(amt: f32) -> Self {
        TroopAmt{
            count: amt as f32,
            disabled: 0.0,
        }
    }
}
