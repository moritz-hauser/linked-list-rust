// Erweiterung von my_linked_list.rs für beliebige Datentypen
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

pub struct Node<T> {
    info: T,
    next: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None, size: 0 }
    }

    pub fn push(&mut self, value: T) {
        let new_node = Node { 
            info: value, 
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.info
        })
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.info)
    }
} 

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node) = current {
            current = node.next.take();
        }
    }
}

pub fn run() {
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Person {
        age: u8,
        name: String,
    }
    
    let mut list = LinkedList::new();
    
    println!("Neue LinkedList erstellt.");
    println!("Aktuelle Länge: {}", list.len());
    
    println!("Person Objekte hinzufügen: Alice, Bob, Charlie");
    list.push( Person{ age: 20, name: String::from("Alice") });
    list.push( Person{ age: 20, name: String::from("Bob") });
    list.push( Person{ age: 20, name: String::from("Charlie") });
    
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