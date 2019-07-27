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

        current_next.map(|nn_next_node| {
            let mut boxed_next_node = unsafe { Box::from_raw(nn_next_node.as_ptr()) };

            self.next = boxed_next_node.next.take();

            boxed_next_node.element.take().unwrap()
        })
    }

    fn peek_next(&self) -> Option<&T> {
        self.next.map(|nn_node| {
            let p_node = nn_node.as_ptr();

            unsafe { (*p_node).element.as_ref().unwrap() }
        })
    }
}

impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        println!("Dropping Node!");

        let _ = self.take_next();
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
        let r_tail = unsafe { self.tail.as_mut() };

        let nn_new_node = r_tail.set_next(element);

        self.tail = nn_new_node;

        self.len += 1;
    }

    pub fn push(&mut self, element: T) {
        if self.len == 0 {
            self.add(element);
        } else {
            let r_head = unsafe { self.head.as_mut() };

            r_head.set_next(element);

            self.len += 1;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let r_head = unsafe { self.head.as_mut() };
        let popped_node = r_head.take_next();

        self.len -= 1;

        if self.len == 0 {
            self.tail = self.head;
        }

        popped_node
    }

    pub fn peek_head(&self) -> Option<&T> {
        let r_head = unsafe { self.head.as_ref() };

        r_head.peek_next()
    }

    pub fn peek_tail(&self) -> Option<&T> {
        let r_tail = unsafe { self.tail.as_ref() };

        r_tail.element.as_ref()
    }
}

impl<T> Drop for SingleLinkedList<T> {
    fn drop(&mut self) {
        println!("Dropping SingleLinkedList!");

        let _ = unsafe { Box::from_raw(self.head.as_ptr()) };
    }
}
