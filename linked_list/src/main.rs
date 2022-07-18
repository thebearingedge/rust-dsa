mod linked_list;

use linked_list::LinkedList;

fn main() {
    let mut ints = LinkedList::new([1, 2, 3, 4, 5].into_iter());

    ints.append(6);
    ints.append(7);
    ints.append(8);
    ints.prepend(0);

    println!("\ninteger list: {:?}", ints);

    let mut words = LinkedList::new(["let's", "learn", "rust"].into_iter());

    for word in &words {
        println!("{:?}", word);
    }

    words.clear();
}
