pub struct LinkedList<T> {
    head: Link<T>,
}
struct Node<T> {
    element: T,
    next: Link<T>,
}
type Link<T> = Option<Box<Node<T>>>;

impl<T> LinkedList<T> {
    pub fn empty() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, element: T) {
        // let old_head = std::mem::replace(&mut self.head, None); //this is same as self.head.take();
        let new_head = Box::new(Node {
            element: element,
            next: self.head.take(),
        });
        self.head = Some(new_head);
    }

    pub fn pop(&mut self) -> Option<T> {
        // let old_head = self.head.take();
        // match old_head {
        //     Some(n) => {
        //         self.head = n.next;
        //         Some(n.element)
        //     }
        //     None => None,
        // } // this is same as old_head.map(...)
        self.head.take().map(|n| {
            self.head = n.next;
            n.element
        })
    }

    pub fn peek(&self) -> Option<&T> {
        // match &self.head {
        //     Some(n) => Some(&n.element),
        //     None => None,
        // }
        self.head.as_ref().map(|n| &n.element)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|n| &mut n.element)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_ref().map(|node| &node)}
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

pub struct IntoIter<T>(LinkedList<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'iter, T> Iterator for Iter<'iter, T> {
    type Item = &'iter T;

    fn next(&mut self) -> Option<Self::Item> {
        

        match self.next {
            Some(n) => {
                self.next = match &n.next {
                    Some(n) => Some(n.as_ref()),
                    None => None,
                };
                Some(&n.element)
            }
            None => {
                None
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let mut ll = LinkedList::empty();

        assert_eq!(ll.pop(), None);
        assert_eq!(ll.peek(), None);

        ll.push(1);
        ll.push(2);
        ll.push(3);

        assert_eq!(ll.peek(), Some(&3));
        assert_eq!(ll.pop(), Some(3));
        assert_eq!(ll.peek(), Some(&2));
        assert_eq!(ll.pop(), Some(2));

        ll.push(4);
        ll.push(5);

        assert_eq!(ll.pop(), Some(5));
        assert_eq!(ll.pop(), Some(4));

        assert_eq!(ll.peek(), Some(&1));
        assert_eq!(ll.peek_mut(), Some(&mut 1));
        assert_eq!(ll.pop(), Some(1));
        assert_eq!(ll.pop(), None);
        assert_eq!(ll.peek(), None);

        ll.push(64);
        assert_eq!(ll.peek(), Some(&64));
        assert_eq!(ll.peek_mut(), Some(&mut 64));

        ll.peek_mut().map(|value| *value = 32);

        assert_eq!(ll.peek(), Some(&32));
        assert_eq!(ll.peek_mut(), Some(&mut 32));
    }

    #[test]
    fn into_iter() {
        let mut list = LinkedList::empty();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }
}
