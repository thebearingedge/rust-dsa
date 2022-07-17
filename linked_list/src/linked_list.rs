#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Copy> Node<T> {
    fn boxed(data: T, next: Option<Box<Node<T>>>) -> Box<Self> {
        Box::new(Node { data, next })
    }
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
                .fold(None, |next, data| Some(Node::boxed(data, next))),
        }
    }

    pub fn prepend(&mut self, data: T) {
        self.head = Some(Node::boxed(data, self.head.take()));
    }

    pub fn append(&mut self, data: T) {
        let mut next = &mut self.head;
        while let Some(node) = next {
            next = &mut node.next;
        }
        next.replace(Node::boxed(data, None));
    }
}

impl<T: Copy> Into<Vec<T>> for LinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut values = vec![];
        let mut next = self.head;
        while let Some(node) = next {
            values.push(node.data);
            next = node.next;
        }
        values
    }
}
