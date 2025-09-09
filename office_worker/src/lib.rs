//use crate::OfficeWorker::*;

#[derive(Debug, PartialEq, Eq)]
pub struct OfficeWorker {
    name: String,
    age: u32,
    role: WorkerRole,
}

#[derive(Debug, PartialEq, Eq)]
pub enum WorkerRole {
    Admin,
    User,
    Guest,
}

impl From<&str> for OfficeWorker {
    fn from(value: &str) -> Self {
        todo!()
    }
}

impl From<&str> for WorkerRole {
    fn from(value: &str) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests;
