use crate::ds::linked_lists::single_linked_list::SingleLinkedList;

#[test]
fn can_create_single_linked_list_by_macro() {
    let mut sll = single_linked_list![1, 4, 5, 9, 3];

    assert_eq!(sll.front(), Some(&1));
    assert_eq!(sll.back(), Some(&3));
    assert_eq!(sll.front(), Some(&1));
    assert_eq!(sll.back(), Some(&3));

    sll.clear();

    assert_eq!(sll.front(), None);
    assert_eq!(sll.back(), None);
    assert_eq!(sll.is_empty(), true);
}

#[test]
fn can_push_front_and_pop_front() {
    let mut sll = single_linked_list![1, 4, 5, 9, 3];

    assert_eq!(sll.front(), Some(&1));
    assert_eq!(sll.back(), Some(&3));
    assert_eq!(sll.is_empty(), false);
    assert_eq!(sll.len(), 5);

    sll.push_front(2);

    assert_eq!(sll.front(), Some(&2));
    assert_eq!(sll.back(), Some(&3));

    sll.push_front(6);

    assert_eq!(sll.front(), Some(&6));
    assert_eq!(sll.back(), Some(&3));

    let elem = sll.pop_front();

    assert_eq!(elem, Some(6));
    assert_eq!(sll.front(), Some(&2));
    assert_eq!(sll.back(), Some(&3));

    let elem = sll.pop_front();

    assert_eq!(elem, Some(2));
    assert_eq!(sll.front(), Some(&1));
    assert_eq!(sll.back(), Some(&3));

    let elem = sll.pop_front();

    assert_eq!(elem, Some(1));
    assert_eq!(sll.front(), Some(&4));
    assert_eq!(sll.back(), Some(&3));

    let elem = sll.pop_front();

    assert_eq!(elem, Some(4));
    assert_eq!(sll.front(), Some(&5));
    assert_eq!(sll.back(), Some(&3));

    let elem = sll.pop_front();

    assert_eq!(elem, Some(5));
    assert_eq!(sll.front(), Some(&9));
    assert_eq!(sll.back(), Some(&3));

    let elem = sll.pop_front();

    assert_eq!(elem, Some(9));
    assert_eq!(sll.front(), Some(&3));
    assert_eq!(sll.back(), Some(&3));

    let elem = sll.pop_front();

    assert_eq!(elem, Some(3));
    assert_eq!(sll.front(), None);
    assert_eq!(sll.back(), None);
    assert_eq!(sll.is_empty(), true);
}

#[test]
fn can_iter() {
    let result_vec = vec![2, 1, 4, 5, 9, 3];
    let mut sll = single_linked_list![1, 4, 5, 9, 3];

    sll.push_front(2);

    assert_eq!(sll.front(), Some(&2));
    assert_eq!(sll.back(), Some(&3));

    sll.push_front(6);

    assert_eq!(sll.front(), Some(&6));
    assert_eq!(sll.back(), Some(&3));

    let elem = sll.pop_front();

    assert_eq!(elem, Some(6));
    assert_eq!(sll.front(), Some(&2));
    assert_eq!(sll.back(), Some(&3));

    let mut i = 0;
    for elem in sll {
        assert_eq!(elem, result_vec[i]);
        i += 1;
    }
}

#[test]
fn can_ref_iter() {
    let result_vec = vec![1, 4, 5, 9, 3];
    let mut sll = single_linked_list![1, 4, 5, 9, 3];

    let mut_ref_sll = &mut sll;

    let mut i = 0;
    for elem in mut_ref_sll{
        assert_eq!(elem, &result_vec[i]);
        i+=1;
    }

    let ref_sll = &sll;

    let mut i = 0;
    for elem in ref_sll{
        assert_eq!(elem, &result_vec[i]);
        i+=1;
    }
}

#[test]
fn can_reverse(){
    let mut result_vec = vec![1, 4, 5, 9, 3];
    result_vec.reverse();
    let mut sll = single_linked_list![1, 4, 5, 9, 3];

    sll.reverse();

    let mut i = 0;
    for elem in sll {
        assert_eq!(elem, result_vec[i]);
        i += 1;
    }
}