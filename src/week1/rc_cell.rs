use std::{cell::RefCell, rc::Rc};

pub struct Stack<T> {
    pub stack: Rc<RefCell<Vec<T>>>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            stack: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn push(&self, a: T) {
        self.stack.borrow_mut().push(a)
    }

    pub fn pop(&self) -> Option<T> {
        self.stack.borrow_mut().pop()
    }
}

impl<T> Clone for Stack<T> {
    fn clone(&self) -> Self {
        Stack {
            stack: Rc::clone(&self.stack),
        }
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Borrow;

    use super::*;

    #[test]
    fn test_push_pop() {
        let s = Stack::new();
        s.push(1);
        s.push(2);
        assert_eq!(s.pop(), Some(2))
    }

    #[test]
    fn test_can_mutate_with_multiple_refs() {
        let s = Stack::new();
        let s1 = s.borrow();
        let s2 = s.borrow();
        s1.push(1);
        s2.push(2);
        assert_eq!(s.pop(), Some(2))
    }

    #[test]
    fn test_cloning_doesnt_clones_data() {
        let s = Stack::new();
        let s1 = s.clone();
        s.push(1);
        s1.push(2);
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s1.pop(), Some(1))
    }
}
