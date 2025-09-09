#[derive(Debug, PartialEq, Eq)]
pub struct Outfit {
    pub jacket: Jacket,
    pub hat: Hat,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Jacket {
    Black,
    White,
    Flowers,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Hat {
    Snapback,
    Baseball,
    Fedora,
}


pub fn choose_outfit(
    formality_level: Option<u32>,
    invitation_message: Result<&str, &str>,
) -> Outfit { 
    todo!()
}

#[cfg(test)]
mod tests;
