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

    pub fn peek(&self) -> Option<&i32> {
        self.head.as_ref().map(|node| &node.info)
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

// ----- TESTS -----
pub fn run() {
    let mut list = LinkedList::new();
    
    println!("Neue LinkedList erstellt.");
    println!("Aktuelle Länge: {}", list.len());
    
    println!("Elemente hinzufügen: 10, 20, 30");
    list.push(10);
    list.push(20);
    list.push(30);
    
    println!("Aktuelle Länge nach Push: {}", list.len());
    
    println!("Peek auf das erste Element: {:?}", list.peek());
    
    println!("Entferne ein Element: {:?}", list.pop());
    println!("Peek nach Pop: {:?}", list.peek());
    println!("Aktuelle Länge nach Pop: {}", list.len());
    
    println!("Entferne ein weiteres Element: {:?}", list.pop());
    println!("Peek nach Pop: {:?}", list.peek());
    
    println!("Entferne ein weiteres Element: {:?}", list.pop());
    println!("Peek nach Pop (Liste sollte leer sein): {:?}", list.peek());
    
    println!("Finale Länge: {}", list.len());
}