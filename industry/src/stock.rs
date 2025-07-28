pub struct Stock {
    name: String,
    value: u32
}

impl stocks {
    pub fn new(name: String, value: u32) -> Stock {
        Stock {
            name,
            value
        }
    }
}
//Health, Entertainment, Defense, Technology, Business, Manufacturing