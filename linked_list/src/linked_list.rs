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
    pub fn new(head: Option<T>) -> Self {
        LinkedList {
            head: head.map(|data| Node::boxed(data, None)),
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

impl<T, const N: usize> From<[T; N]> for LinkedList<T> {
    fn from(array: [T; N]) -> Self {
        LinkedList {
            head: array
                .into_iter()
                .rfold(None, |next, data| Some(Node::boxed(data, next))),
        }
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
        let empty: LinkedList<i32> = LinkedList::new(None);
        assert_eq!(empty, LinkedList::from([]));
        let non_empty = LinkedList::new(Some(42));
        assert_eq!(non_empty, LinkedList::from([42]))
    }

    #[test]
    fn test_append() {
        let mut list = LinkedList::from([1, 2, 3]);
        list.append(4);
        assert_eq!(list, LinkedList::from([1, 2, 3, 4]));
    }

    #[test]
    fn test_prepend() {
        let mut list = LinkedList::from([1, 2, 3]);
        list.prepend(0);
        assert_eq!(list, LinkedList::from([0, 1, 2, 3]));
    }
}
