#[derive(Debug)]
enum LinkedList<Data> {
    Empty,
    Node(Data, Box<LinkedList<Data>>),
}

impl<D> LinkedList<D> {
    fn new<I>(values: I) -> LinkedList<D>
    where
        I: DoubleEndedIterator<Item = D>,
    {
        values.rev().fold(LinkedList::Empty, |succ, val| {
            LinkedList::Node(val, Box::new(succ))
        })
    }
}

fn main() {
    let list = LinkedList::new(vec![1, 2, 3, 4, 5].into_iter());
    println!("Hello, world!");
    println!("{:?}", list);
}
