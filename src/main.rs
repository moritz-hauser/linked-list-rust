mod my_linked_list;
mod generic_linked_list;

fn main() {
    println!("--- START: my_linked_list --- ");
    my_linked_list::run();
    println!("--- ENDE: my_linked_list --- ");

    println!("--- START: generic_linked_list --- ");
    generic_linked_list::run();
    println!("--- ENDE: generic_linked_list --- ");
}
