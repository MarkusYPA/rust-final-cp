use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Table {
	pub headers: Vec<String>,
	pub body: Vec<Vec<String>>,
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}

impl Table {
	pub fn new() -> Table {
        todo!()
	}
	pub fn add_row(&mut self, row: &[String]) {
        todo!()
	}
}

#[cfg(test)]
mod tests;
