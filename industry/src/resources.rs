#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum Resource {
    Food,
    Metal,
    Oil
}

impl Resource {
    pub const ALL: [Resource; 3] = [
        Resource::Food,
        Resource::Metal,
        Resource::Oil 
    ];
}