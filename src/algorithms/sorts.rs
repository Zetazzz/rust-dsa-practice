use std::cmp::Ordering;

//not test yet
pub fn fast_sort(arr: &mut [u64]) {
    if arr.len() == 0 {
        return;
    }
    if arr.len() == 1 {
        return;
    }

    let pivot = arr[0];
    let mut mid = 0;
    let mut i = 1;

    while i < arr.len() {
        if let Ordering::Less = arr[i].cmp(&pivot) {
            mid += 1;
            swap(arr, mid, i);
        }

        i += 1;
    }

    swap(arr, 0, mid);
    fast_sort(&mut arr[0..mid]);
    fast_sort(&mut arr[mid + 1..]);
}

fn swap(arr: &mut [u64], a: usize, b: usize) {
    let temp = arr[a];
    arr[a] = arr[b];
    arr[b] = temp;
}
