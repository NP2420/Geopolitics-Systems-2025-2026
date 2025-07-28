pub struct Territory {
    name: String,
    pub tier: u32, //1-3
    pub population: u32 //1-10
}

impl Territory {
    pub fn new(name: String, tier: u32, population: u32) -> Territory {
        Territory {
            name,
            tier,
            population
        }
    }
}