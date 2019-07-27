use std::ptr::NonNull;

#[macro_export]
macro_rules! single_linked_list {
    [$($elem:expr,)+]=>{
        single_linked_list![$($elem),+]
    };
    [$($elem:expr),*]=>{
        {
            let mut sll = SingleLinkedList::new();

            $(
                sll.add($elem);
            )*

            sll
        }
    }
}

pub struct Node<T> {
    element: Option<T>,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(element: T, next: Option<NonNull<Node<T>>>) -> Self {
        Node {
            element: Some(element),
            next: next,
        }
    }

    fn empty() -> Self {
        Node {
            element: None,
            next: None,
        }
    }

    fn set_next(&mut self, element: T) -> NonNull<Node<T>> {
        let current_next = self.next.take();

        let nn_node = Box::into_raw_non_null(Box::new(Node::new(element, current_next)));

        self.next = Some(nn_node);

        nn_node
    }

    fn take_next(&mut self) -> Option<T> {
        let current_next = self.next.take();

        current_next.map(|nn_node| {
            let boxed_node = unsafe { Box::from_raw(nn_node.as_ptr()) };

            self.next = boxed_node.next;

            boxed_node.element.unwrap()
        })
    }

    fn peek_next(&self) -> Option<T> {
        self.next.map(|nn_node| {
            let boxed_node = unsafe { Box::from_raw(nn_node.as_ptr()) };

            boxed_node.element.unwrap()
        })
    }
}

pub struct SingleLinkedList<T> {
    head: NonNull<Node<T>>,
    tail: NonNull<Node<T>>,
    len: usize,
}

impl<T> SingleLinkedList<T> {
    pub fn new() -> Self {
        let nn_node = Box::into_raw_non_null(Box::new(Node::empty()));

        SingleLinkedList {
            head: nn_node,
            tail: nn_node,
            len: 0,
        }
    }

    pub fn add(&mut self, element: T) {
        let mut boxed_node = unsafe { Box::from_raw(self.tail.as_ptr()) };

        let nn_new_node = boxed_node.set_next(element);

        self.tail = nn_new_node;

        self.len += 1;
    }

    pub fn push(&mut self, element: T) {
        if self.len == 0 {
            self.add(element);
        } else {
            let mut boxed_node = unsafe { Box::from_raw(self.head.as_ptr()) };

            boxed_node.set_next(element);

            self.len += 1;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut boxed_node = unsafe { Box::from_raw(self.head.as_ptr()) };
        let popped_node = boxed_node.take_next();

        self.len -= 1;

        if self.len == 0 {
            self.tail = self.head;
        }

        popped_node
    }

    pub fn peek_head(&self) -> Option<T> {
        let boxed_node = unsafe { Box::from_raw(self.head.as_ptr()) };

        boxed_node.peek_next()
    }

    pub fn peek_tail(&self) -> Option<T> {
        let boxed_node = unsafe { Box::from_raw(self.tail.as_ptr()) };

        boxed_node.element
    }
}
