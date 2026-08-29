use linked_list::prelude::*;

fn main() {
    let mut ll = LinkedList::default();

    ll.push(3);
    ll.push(3);
    ll.push(7);

    println!("{:#?}", ll);
}
