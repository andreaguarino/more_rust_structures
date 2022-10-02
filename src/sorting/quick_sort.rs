pub fn quick_sort<T: Ord + Clone>(list: &mut Vec<T>) {
    quick_sort_rec(list);
}

fn quick_sort_rec<T: Ord + Clone>(list: &mut [T]) {
    if list.len() <= 1 {
        return;
    }
    let pivot = list[list.len() / 2].clone();
    let mut i = 0;
    let mut j = list.len() - 1;
    loop {
        while list[i] < pivot {
            i += 1;
        }
        while list[j] > pivot {
            j -= 1;
        }
        if i == j {
            break;
        }
        list.swap(i, j);
    }
    quick_sort_rec(&mut list[..j]);
    quick_sort_rec(&mut list[j + 1..]);
}
