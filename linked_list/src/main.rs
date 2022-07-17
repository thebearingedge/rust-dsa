mod linked_list;

use linked_list::LinkedList;

fn main() {
    let mut int_list = LinkedList::new([1, 2, 3, 4, 5].into_iter());
    int_list.append(6);
    int_list.append(7);
    int_list.append(8);
    int_list.prepend(0);
    println!("\ninteger list: {:?}", int_list);
    let str_list = LinkedList::new(["let's", "learn", "rust"].into_iter());
    println!("\nslice list: {:?}", str_list);
}
