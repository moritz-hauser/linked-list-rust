/*
TODO:
- Traits: Debug, Copy, Clone, Display
- Für generische Datentypen
*/
pub struct LinkedList {
    head: Option<Box<Node>>,
    size: usize,
}

pub struct Node {
    info: i32,
    next: Option<Box<Node>>,
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList { head: None, size: 0 }
    }

    pub fn push(&mut self, value: i32) {
        let new_node = Node { 
            info: value, 
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.info
        })
    }

    pub fn len(&self) -> usize {
        self.size
    }
} 

impl Drop for LinkedList {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node) = current {
            current = node.next.take();
        }
    }
}