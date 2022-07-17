mod linked_list;

use linked_list::LinkedList;

fn main() {
    let int_list = LinkedList::new([1, 2, 3, 4, 5].into_iter());
    println!("integer list: {:?}", int_list);
    let str_list = LinkedList::new(["let's", "learn", "rust"].into_iter());
    println!("slice list: {:?}", str_list);
}
