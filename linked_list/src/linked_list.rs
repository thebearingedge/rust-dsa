#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct LinkedList<T: Copy> {
    head: Option<Box<Node<T>>>,
}

impl<T: Copy> LinkedList<T> {
    pub fn new<I>(values: I) -> LinkedList<T>
    where
        I: DoubleEndedIterator<Item = T>,
    {
        LinkedList {
            head: values
                .rev()
                .fold(None, |next, value| Some(Box::new(Node { value, next }))),
        }
    }
}

impl<T: Copy> Into<Vec<T>> for LinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut values = vec![];
        let mut next = self.head;
        while let Some(node) = next {
            values.push(node.value);
            next = node.next;
        }
        values
    }
}
