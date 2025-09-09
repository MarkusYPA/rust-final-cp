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

	pub fn filter_col<T>(&self, filter: T) -> Option<Self> {
        todo!()
	}

	pub fn filter_row<T>(&self, col_name: &str, filter: T) -> Option<Self> {
        todo!()
	}
}

#[cfg(test)]
mod tests;
