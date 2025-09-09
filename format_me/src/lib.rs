use std::fmt;

pub struct Park {
    pub name: String,
    pub park_type: ParkType,
    pub address: String,
    pub cap: String,
    pub state: String,
}

pub enum ParkType {
    Garden,
    Forest,
    Playground,
}

impl fmt::Display for Park {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl fmt::Display for ParkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[cfg(test)]
mod tests;
