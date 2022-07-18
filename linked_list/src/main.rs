mod linked_list;

use linked_list::LinkedList;

fn main() {
    let mut ints = LinkedList::new(None);

    ints.append(6);
    ints.append(7);
    ints.append(8);
    ints.prepend(0);

    println!("\ninteger list: {:?}", ints);

    let mut words = LinkedList::from(["let's", "learn", "rust"]);

    for word in &words {
        println!("{:?}", word);
    }

    words.clear();
}
