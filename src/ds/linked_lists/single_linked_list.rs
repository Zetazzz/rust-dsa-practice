use std::marker::PhantomData;
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
                sll.push_back($elem);
            )*

            sll
        }
    }
}

pub struct Node<T> {
    element: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(element: T, next: Option<NonNull<Node<T>>>) -> Self {
        Node {
            element: element,
            next: next,
        }
    }

    pub fn push_next(&mut self, element: T) -> NonNull<Node<T>> {
        let current_next = self.next.take();

        let nn_node = Box::into_raw_non_null(Box::new(Node::new(element, current_next)));

        self.next = Some(nn_node);

        nn_node
    }

    pub fn take_next(&mut self) -> Option<T> {
        let current_next = self.next.take();

        current_next.map(|nn_next_node| {
            let mut boxed_next_node = unsafe { Box::from_raw(nn_next_node.as_ptr()) };

            self.next = boxed_next_node.next.take();

            boxed_next_node.element
        })
    }

    pub fn peek_next(&self) -> Option<&T> {
        self.next.map(|nn_node| {
            let p_node = nn_node.as_ptr();

            unsafe { &(*p_node).element }
        })
    }

    pub fn has_next(&self) -> bool {
        self.next.is_some()
    }
}

pub struct SingleLinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    mark: PhantomData<Box<Node<T>>>,
}

impl<T> SingleLinkedList<T> {
    pub fn new() -> Self {
        SingleLinkedList {
            head: None,
            tail: None,
            len: 0,
            mark: PhantomData,
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            head: self.head,
            len: self.len,
            mark: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            head: self.head,
            len: self.len,
            mark: PhantomData,
        }
    }

    pub fn cursor(&self) -> Cursor<T> {
        Cursor { current: self.head }
    }

    pub fn reverse(&mut self) {
        let mut cursor = self.cursor();

        if let Some(first_node) = cursor.current() {
            while let Some(next_element) = first_node.take_next() {
                self.push_front(next_element);
            }
        }
    }

    pub fn push_back(&mut self, element: T) {
        match self.tail {
            Some(mut nn_tail) => {
                let r_tail = unsafe { nn_tail.as_mut() };

                let nn_new_node = r_tail.push_next(element);

                self.tail = Some(nn_new_node);
            }
            None => {
                self.add_first_node(element);
            }
        }

        self.len += 1;
    }

    pub fn push_front(&mut self, element: T) {
        match self.head {
            Some(nn_head) => {
                let nn_new_node =
                    Box::into_raw_non_null(Box::new(Node::new(element, Some(nn_head))));

                self.head = Some(nn_new_node);
            }
            None => {
                self.add_first_node(element);
            }
        }

        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.map(|nn_head| {
            let mut boxed_taken_head = unsafe { Box::from_raw(nn_head.as_ptr()) };

            self.head = boxed_taken_head.next.take();

            if let None = self.head {
                self.tail = None;
            }

            boxed_taken_head.element
        })
    }

    fn add_first_node(&mut self, element: T) {
        let nn_new_node = Box::into_raw_non_null(Box::new(Node::new(element, None)));

        self.head = Some(nn_new_node);
        self.tail = Some(nn_new_node);
    }

    pub fn front(&self) -> Option<&T> {
        unsafe { self.head.as_ref().map(|node| &node.as_ref().element) }
    }

    pub fn front_mut(&mut self) -> Option<&T> {
        unsafe { self.head.as_mut().map(|node| &node.as_mut().element) }
    }

    pub fn back(&self) -> Option<&T> {
        unsafe { self.tail.as_ref().map(|node| &node.as_ref().element) }
    }

    pub fn back_mut(&mut self) -> Option<&T> {
        unsafe { self.tail.as_mut().map(|node| &node.as_mut().element) }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn clear(&mut self) {
        *self = Self::new();
    }
}

impl<T> Drop for SingleLinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_front() {}
    }
}

pub struct Iter<'a, T: 'a> {
    head: Option<NonNull<Node<T>>>,
    len: usize,
    mark: PhantomData<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.head.map(|node| {
                let node = unsafe { &*node.as_ptr() };
                self.len -= 1;
                self.head = node.next;
                &node.element
            })
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

pub struct IterMut<'a, T: 'a> {
    head: Option<NonNull<Node<T>>>,
    len: usize,
    mark: PhantomData<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.head.map(|node| {
                let node = unsafe { &mut *node.as_ptr() };
                self.len -= 1;
                self.head = node.next;
                &mut node.element
            })
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

pub struct IntoIter<T> {
    linked_list: SingleLinkedList<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.linked_list.pop_front()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.linked_list.len, Some(self.linked_list.len))
    }
}

impl<T> IntoIterator for SingleLinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { linked_list: self }
    }
}

impl<'a, T> IntoIterator for &'a SingleLinkedList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut SingleLinkedList<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

pub struct Cursor<T> {
    current: Option<NonNull<Node<T>>>,
}

impl<T> Cursor<T> {
    pub fn current(&mut self) -> Option<&mut Node<T>> {
        self.current.map(|node| unsafe { &mut *node.as_ptr() })
    }

    pub fn push_next(&mut self, element: T) {
        if let Some(mut node) = self.current {
            unsafe {
                node.as_mut().push_next(element);
            }
        }
    }

    pub fn pop_next(&mut self) -> Option<T> {
        if let Some(mut node) = self.current {
            unsafe { node.as_mut().take_next() }
        } else {
            None
        }
    }
}

impl<T> Iterator for Cursor<T> {
    type Item = NonNull<Node<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(node) => {
                let node = unsafe { node.as_ref() };
                self.current = node.next;
                node.next
            }
            None => None,
        }
    }
}
