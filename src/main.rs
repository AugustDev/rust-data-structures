mod data_structures;

use data_structures::linked_list::LinkedList;

fn main() {
    println!("Hello, world!");
    let x = LinkedList { val: 3 };
    println!("{}", x.val);
}
