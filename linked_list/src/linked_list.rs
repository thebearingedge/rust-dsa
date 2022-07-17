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
    pub fn new<I>(values: I) -> Self
    where
        I: DoubleEndedIterator<Item = T>,
    {
        LinkedList {
            head: values
                .rev()
                .fold(None, |next, value| Some(Box::new(Node { value, next }))),
        }
    }

    pub fn append(&mut self, value: T) {
        let mut next = &mut self.head;
        while let Some(node) = next {
            next = &mut node.next;
        }
        let _ = std::mem::replace(next, Some(Box::new(Node { value, next: None })));
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
