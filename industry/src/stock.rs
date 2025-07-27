pub struct stock {
    name: String,
    value: u32
}

impl stocks {
    pub fn new(name: String, value: u32) -> stock {
        stock {
            name,
            value
        }
    }
}
//Health, Entertainment, Defense, Technology, Business, Manufacturing