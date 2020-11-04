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

    fn peek(&self) -> Option<&T> {
        match &self.head {
            Some(n) => Some(&n.element),
            None => None,
        }
        // self.head.as_ref().map(|n| &n.element) //this is same, less readable tho
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Link::Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
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
        assert_eq!(ll.pop(), Some(1));
        assert_eq!(ll.pop(), None);
        assert_eq!(ll.peek(), None);
    }
}
