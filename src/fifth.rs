// 6. An Unsafe Singly-Linked Queue

// Layout:
// flipped push, instead of flipped pop
//   input list:
//   [Some(ptr)] -> (A, Some(ptr)) -> (B, None)
//   [Some(ptr):tail] ----------------^
//
//   flipped push X:
//   [Some(ptr)] -> (A, Some(ptr)) -> (B, Some(ptr)) -> (X, None)
//   [Some(ptr):tail] ----------------------------------^

// Invariants:
// - head = None <=> tail = None

pub struct List<'a, T: 'a> {
    head: Link<T>,
    tail: Option<&'a mut Node<T>>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}


impl<'a, T> List<'a, T> {
    pub fn new() -> Self {
        List{ head: None, tail: None }
    }

    pub fn push(&'a mut self, elem: T) {
        let new_tail_node = Box::new(Node{
            elem: elem,
            next: None });
        let new_tail_ref = match self.tail.take() {
            Some(old_tail) => {
                old_tail.next = Some(new_tail_node);
                old_tail.next.as_mut().map(|node| &mut **node)
            }
            None => {
                // zero elements, update head too
                self.head = Some(new_tail_node);
                self.head.as_mut().map(|node| &mut **node)
            }
        };
        self.tail = new_tail_ref;
    }

    //   input list:
    //   [Some(ptr)] ------> (A, Some(ptr)) -> (B, None)
    //                                         &
    //   [Some(ptr):tail] ---------------------^
    //
    //   pop -> Some(A)
    //   [Some(ptr)] ------------------------> (B, None)
    //                                         &
    //   [Some(ptr):tail] ---------------------^
    pub fn pop(&'a mut self) -> Option<T> {
        self.head.take().map(|boxed_head| {
            // Deref head out of box, own it.
            let head = *boxed_head;
            self.head = head.next;

            // If new head is none, tail should be none too as well.
            if self.head.is_none() {
                self.tail = None;
            }

            head.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);

        list.push(1); list.push(2); list.push(3);

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        list.push(4); list.push(5);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);
    }
}
