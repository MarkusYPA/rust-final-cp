pub struct Field {
    head: Link,
}

type Link = Option<Box<Node>>; 

struct Node {
    elem: Target,
    next: Link,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Target {
    pub size: u32,
    pub xp: u32,
}
impl Field {
    pub fn new() -> Self {
        todo!()
    }
    
    pub fn push(&mut self, target: Target) {
        todo!()
    }

    pub fn pop(&mut self) -> Option<Target> {
        todo!()
    }

    pub fn peek(&self) -> Option<&Target> {
        todo!()
    }

    pub fn peek_mut(&mut self) -> Option<&mut Target> {
        todo!()
    }
}

#[cfg(test)]
mod tests;
