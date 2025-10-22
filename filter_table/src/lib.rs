#[derive(Clone, Debug, PartialEq)]
pub struct Table {
	pub headers: Vec<String>,
	pub body: Vec<Vec<String>>,
}

impl Table {
	pub fn new() -> Table {
        todo!()
	}

	pub fn add_row(&mut self, row: &[String]) {
        todo!()
	}

	//pub fn filter_col<T>(&self, filter: T) -> Option<Self> {
	pub fn filter_col<T: Fn(&str) -> bool>(&self, filter: T) -> Option<Self> {	// T defined so tests don't throw compilation error
        todo!()
	}

	//pub fn filter_row<T>(&self, col_name: &str, filter: T) -> Option<Self> {
	pub fn filter_row<T: Fn(&str) -> bool>(&self, col_name: &str, filter: T) -> Option<Self> {
        todo!()
	}
}

#[cfg(test)]
mod tests;