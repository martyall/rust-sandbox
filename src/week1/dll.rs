use std::{cell::RefCell, rc::Rc};

struct ListNode<T> {
    item: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> ListNode<T> {
    fn new(item: T) -> Self {
        Self {
            item,
            next: None,
            prev: None,
        }
    }
}

type Link<T> = Option<Rc<RefCell<ListNode<T>>>>;

pub struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, item: T) {
        let n = ListNode::new(item);
        let new_head = Rc::new(RefCell::new(n));
        match self.head.take() {
            None => {
                self.head = Some(Rc::clone(&new_head));
                self.tail = Some(new_head);
            }
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::clone(&new_head));
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(current_head) => {
                match &current_head.borrow().next {
                    // our list only contained one element
                    None => {
                        self.tail = None;
                        self.head = None;
                    }
                    Some(new_head) => {
                        new_head.borrow_mut().prev = None;
                        self.head = Some(Rc::clone(new_head));
                    }
                };
                println!(
                    "popping head with current Rc count {}",
                    Rc::strong_count(&current_head)
                );
                Some(Rc::into_inner(current_head).unwrap().into_inner().item)
            }
        }
    }

    pub fn push_back(&mut self, item: T) {
        let new_tail = Rc::new(RefCell::new(ListNode::new(item)));
        match self.tail.take() {
            None => {
                self.head = Some(Rc::clone(&new_tail));
                self.tail = Some(new_tail);
            }
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(Rc::clone(&new_tail));
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        match self.tail.take() {
            None => None,
            Some(current_tail) => {
                match &current_tail.borrow().prev {
                    None => {
                        self.head = None;
                        self.tail = None;
                    }
                    Some(new_tail) => {
                        new_tail.borrow_mut().next = None;
                        self.tail = Some(Rc::clone(new_tail));
                    }
                }

                println!(
                    "popping tail with current Rc count {}",
                    Rc::strong_count(&current_tail)
                );

                Some(Rc::into_inner(current_tail).unwrap().into_inner().item)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_push_front() {
        let v = vec![1, 2, 3];
        let mut ll = DoublyLinkedList::new();
        for elem in v.into_iter() {
            ll.push_front(elem);
        }
        let a = ll.pop_front();
        assert_eq!(a, Some(3));
        let a = ll.pop_front();
        assert_eq!(a, Some(2));
        let a = ll.pop_front();
        assert_eq!(a, Some(1));
    }

    #[test]
    fn test_push_back() {
        let v = vec![1, 2, 3];
        let mut ll = DoublyLinkedList::new();
        for elem in v.into_iter() {
            ll.push_back(elem);
        }
        let a = ll.pop_back();
        assert_eq!(a, Some(3));
        let a = ll.pop_back();
        assert_eq!(a, Some(2));
        let a = ll.pop_back();
        assert_eq!(a, Some(1));
    }
}
