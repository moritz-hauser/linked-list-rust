// Erweiterung von my_linked_list.rs um immutable / mutable / owning iterators
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

    pub fn iter(&self) -> Iter {
        Iter { current: self.head.as_deref() }
    }

    pub fn iter_mut(&mut self) -> IterMut {
        IterMut { current: self.head.as_deref_mut() }
    }

    pub fn into_iter(self) -> IntoIter {
        IntoIter { current: self.head }
    }
}

// ----- IMMUTABLE ITERATOR -----
pub struct Iter<'a> {
    current: Option<&'a Node>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| {
            self.current = node.next.as_deref();
            &node.info
        })
    } 
}

// ----- MUTABLE ITERATOR -----
pub struct IterMut<'a> {
    current: Option<&'a mut Node>,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node| {
            self.current = node.next.as_deref_mut();
            &mut node.info
        })
    }
}

// ----- OWNING ITERATOR -----
pub struct IntoIter {
    current: Option<Box<Node>>,
}

impl Iterator for IntoIter {
    type Item = i32;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node| {
            self.current = node.next;
            node.info
        })
    }
}

// ----- TESTS -----
pub fn run() {
    let mut list = LinkedList::new();
    
    println!("Neue LinkedList erstellt.");
    
    println!("Elemente hinzufügen: 10, 20, 30");
    list.push(10);
    list.push(20);
    list.push(30);

    println!("Immutable Iterator über alle Elemente:");
    list.iter().for_each(|i| println!("{:?}", i));
    
    println!("Mutable Iterator verdoppelt alle Elemente:");
    list.iter_mut().for_each(|i| *i *= 2);
    for i in list.iter() {
        println!("{:?}", i);
    }

    println!("Owning Iterator über alle Elemente:");
    list.into_iter().for_each(|i| println!("{:?}", i));

    // list.push(10); // Nicht möglich, weil into_iter() Ownership übernimmt
}