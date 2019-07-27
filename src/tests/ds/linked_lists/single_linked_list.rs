#[macro_use]
use crate::ds::linked_lists::single_linked_list::SingleLinkedList;

#[test]
fn can_create_single_linked_list_by_macro() {
    let sll = single_linked_list![1, 4, 5, 9, 3];

    assert_eq!(sll.peek_head(), Some(&1));
    assert_eq!(sll.peek_tail(), Some(&3));
    assert_eq!(sll.peek_head(), Some(&1));
    assert_eq!(sll.peek_tail(), Some(&3));
}
