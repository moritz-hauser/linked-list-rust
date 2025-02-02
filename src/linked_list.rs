// Mock Implementation
#[derive(Debug)]
pub struct LinkedList {
    values: Vec<i32>,
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList { values: Vec::new() }
    }
    
    pub fn push(&mut self, value: i32) {
        self.values.push(value);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.values.pop()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}