use std::iter::FusedIterator;

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

    pub fn tail(mut self) -> Self {
        LinkedList {
            head: match self.head.take() {
                None => None,
                Some(node) => node.next,
            },
        }
    }

    pub fn drop(&mut self) {
        self.head = match self.head.take() {
            None => None,
            Some(node) => node.next,
        };
    }

    pub fn reverse(&mut self) {
        let mut prev = None;
        let mut curr = self.head.take();
        while let Some(mut node) = curr.take() {
            curr = node.next;
            node.next = prev;
            prev = Some(node);
        }
        self.head = prev;
    }

    pub fn iter(&self) -> LinkedListIter<'_, T> {
        LinkedListIter(&self.head)
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

pub struct LinkedListIterator<T>(Option<Box<Node<T>>>);

impl<T> FusedIterator for LinkedListIterator<T> {}

impl<T> Iterator for LinkedListIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.take() {
            None => None,
            Some(node) => {
                self.0 = node.next;
                Some(node.data)
            }
        }
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = LinkedListIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        LinkedListIterator(self.head)
    }
}

pub struct LinkedListIter<'a, T>(&'a Option<Box<Node<T>>>);

impl<'a, T> FusedIterator for LinkedListIter<'a, T> {}

impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            None => None,
            Some(node) => {
                self.0 = &node.next;
                Some(&node.data)
            }
        }
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;
    type IntoIter = LinkedListIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        LinkedListIter(&self.head)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_new() {
        let empty: LinkedList<()> = LinkedList::new(None);
        assert_eq!(empty, LinkedList::from([]));
        let non_empty = LinkedList::new(Some(42));
        assert_eq!(non_empty, LinkedList::from([42]));
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

    #[test]
    fn test_clear() {
        let mut list = LinkedList::from([1, 2, 3]);
        list.clear();
        assert_eq!(list, LinkedList::from([]));
    }

    #[test]
    fn test_tail() {
        let list = LinkedList::from([1, 2, 3]);
        let tail = list.tail();
        assert_eq!(tail, LinkedList::from([2, 3]))
    }

    #[test]
    fn test_drop() {
        let mut list = LinkedList::from([1, 2, 3]);
        list.drop();
        assert_eq!(list, LinkedList::from([2, 3]))
    }

    #[test]
    fn test_reverse() {
        let mut list = LinkedList::from([1, 2, 3, 4, 5]);
        list.reverse();
        assert_eq!(list, LinkedList::from([5, 4, 3, 2, 1]))
    }

    #[test]
    fn test_iter() {
        let list = LinkedList::from([1, 2, 3, 4, 5]);
        let vec = list.iter().collect::<Vec<_>>();
        assert_eq!(vec, vec![&1, &2, &3, &4, &5]);
    }

    #[test]
    fn test_into_iter() {
        let list = LinkedList::from([1, 2, 3, 4, 5]);
        let vec = list.into_iter().collect::<Vec<_>>();
        assert_eq!(vec, vec![1, 2, 3, 4, 5]);
    }
}
