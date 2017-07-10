use std::rc::Rc;
use std::cell::RefCell;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}


impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            next: None,
            prev: None,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        // new node needs +2 links, everything else should be +0
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                                                             // +1 new
                new_head.borrow_mut().next = Some(old_head); // +1 old
                self.head = Some(new_head);                  // +1 new, -1 old
            }
            None => {
                self.head = Some(new_head.clone());          // +1 new
                self.tail = Some(new_head);                  // +1 new
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        // need to take the old head, ensureing it's -1
        self.head.take().map(|old_head| {              // -1 old
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {                    // -1 new
                    new_head.borrow_mut().prev.take(); // -1 old
                    self.head = Some(new_head);        // +1 new
                }
                None => {
                    self.tail = None                   // -1 old
                }
            };
            // Grab old_head by value, because it should be dropped.
            // - `fn try_unwrap(this: Rc<T>) -> Result<T, Rc<T>>`
            //   https://doc.rust-lang.org/std/rc/struct.Rc.html#method.try_unwrap
            // - `Result<T, E>` where E:Debug
            //   - `fn unwrap(self) -> T`
            //   - https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap
            // - `Option<T>`: `fn unwrap(self) -> T`
            //   https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap
            // - `RefCell`: `fn into_inner(self) -> T`
            //   https://doc.rust-lang.org/std/cell/struct.RefCell.html#method.into_inner
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_front(4);
        list.push_front(5);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }
}
