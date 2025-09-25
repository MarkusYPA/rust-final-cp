use std::fmt;

pub struct Park {
    pub name: Option<String>,
    pub park_type: ParkType,
    pub address: Option<String>,
    pub cap: Option<String>,
    pub state: Option<String>,
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
