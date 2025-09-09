#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
	A,
	AB,
	B,
	O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
	Positive,
	Negative,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct BloodType {
	pub antigen: Antigen,
	pub rh_factor: RhFactor,
}

impl BloodType {
	pub fn can_receive_from(&self, other: &Self) -> bool {
        todo!()
	}

	pub fn donors(&self) -> Vec<Self> {
        todo!()
	}

	pub fn recipients(&self) -> Vec<Self> {
        todo!()
	}
}

#[cfg(test)]
mod tests;
