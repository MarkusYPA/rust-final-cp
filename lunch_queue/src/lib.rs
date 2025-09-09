#[derive(Debug)]
pub struct Queue {
    pub node: Link,
}

pub type Link = Option<Box<Person>>;

#[derive(Debug)]
pub struct Person {
    pub discount: i32,
    pub name: String,
}

impl Queue {
    pub fn new() -> Queue {
        todo!()
    }

    pub fn add(&mut self, name: String, discount: i32) {

    }

    pub fn invert_queue(&mut self) {

    }

    pub fn rm(&mut self) -> Option<(String, i32)> {
        todo!()
    }

    pub fn search(&self, name: &str) -> Option<(String, i32)> {
        todo!()
    }
}

#[cfg(test)]
mod tests;
