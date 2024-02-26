pub mod week1;

fn main() {
    println!("Hello, world!");
}

struct Node<T> {
    value: T,
    next: Box<LinkedList<T>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            next: Box::new(LinkedList { head: None }),
        }
    }
}

struct LinkedList<T> {
    head: Option<Node<T>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|x| &x.value)
    }

    pub fn cons(&mut self, value: T) {
        match self.head.take() {
            None => {
                self.head = Some(Node::new(value));
            }
            Some(h) => {
                let mut n = Node::new(value);
                n.next = Box::new(LinkedList { head: Some(h) });
                self.head = Some(n);
            }
        }
    }
}

impl<T> From<Vec<T>> for LinkedList<T> {
    fn from(mut v: Vec<T>) -> Self {
        v.reverse();
        let mut res = LinkedList::new();
        for elem in v {
            res.cons(elem);
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cons() {
        let v = vec![1, 2, 3];
        let ll: LinkedList<i32> = v.into();
        assert_eq!(ll.head.unwrap().value, 1)
    }
}
