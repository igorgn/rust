mod linked_list;

use linked_list::*;

fn main() {
    let mut ll = LinkedList::new();
    ll.insert_end(4);
    ll.insert_end(5);
    ll.insert_start(3);
    ll.insert_start(2);
    ll.insert_start(1);
    ll.insert_start(0);

    let res = ll.search(5);
    println!("Res: {}", match res {
        Some(node) => node.borrow().data,
        None => -1,
    });

    for node in ll {
        println!("{}", node.borrow().data)
    }
}
