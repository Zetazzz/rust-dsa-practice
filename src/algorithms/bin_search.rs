use std::cmp::Ordering;

pub fn bin_search(arr: &[u32], v: u32) -> Option<usize> {
    let mut i = 0;
    let mut j = arr.len() - 1;

    match arr[i].cmp(&v) {
        Ordering::Greater => return None,
        Ordering::Equal => return Some(i),
        Ordering::Less => (),
    }

    match arr[j].cmp(&v) {
        Ordering::Less => return None,
        Ordering::Equal => return Some(j),
        Ordering::Greater => (),
    }

    while i < j - 1 {
        let m = i + (j - i) / 2;
        println!("m: {}", m);

        match arr[m].cmp(&v) {
            Ordering::Greater => {
                j = m;
                println!("j: {}", j);
            }
            Ordering::Equal => return Some(m),
            Ordering::Less => {
                i = m;
                println!("i: {}", i);
            }
        }
    }

    return None;
}