mod linked_list;
use linked_list::LinkedList;
fn main() {
    // Tests für Linked List
    println!("START OF TESTS");

    // push & pop
    let mut list = LinkedList::new();
    list.push(42);
    assert_eq!(list.pop(), Some(42));

    // len
    let mut list = LinkedList::new();
    assert_eq!(0_usize, list.len());
    list.push(100);
    assert_eq!(1_usize, list.len());
    list.push(1);
    list.push(5);
    assert_eq!(3_usize, list.len());

    println!("END OF TESTS");
}
