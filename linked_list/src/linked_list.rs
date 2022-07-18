#[derive(Debug, PartialEq)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn boxed(data: T, next: Option<Box<Node<T>>>) -> Box<Self> {
        Box::new(Node { data, next })
    }
}

#[derive(Debug, PartialEq)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new<I>(values: I) -> Self
    where
        I: DoubleEndedIterator<Item = T>,
    {
        LinkedList {
            head: values.rfold(None, |next, data| Some(Node::boxed(data, next))),
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

    pub fn clear(&mut self) {
        self.head.take();
    }
}

impl<T: Copy> Into<Vec<T>> for LinkedList<T> {
    fn into(self) -> Vec<T> {
        self.into_iter().map(|value| *value).collect::<Vec<T>>()
    }
}

pub struct LinkedListIter<'a, T>(Option<&'a Box<Node<T>>>);

impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            None => None,
            Some(node) => {
                self.0 = node.next.as_ref();
                Some(&node.data)
            }
        }
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;

    type IntoIter = LinkedListIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        LinkedListIter(self.head.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _: LinkedList<i32> = LinkedList::new([].into_iter());
    }

    #[test]
    fn test_append() {
        let mut list = LinkedList::new([1, 2, 3].into_iter());
        list.append(4);
        assert_eq!(list, LinkedList::new([1, 2, 3, 4].into_iter()));
    }
}
