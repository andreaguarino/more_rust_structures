pub fn quick_sort<T: Ord + Clone>(list: &mut Vec<T>) {
    _quick_sort(list);
}

pub fn quick_selection<T: Ord + Clone>(list: &mut Vec<T>, k: usize) -> &T {
    _quick_selection(list, k)
}

fn _quick_sort<T: Ord + Clone>(list: &mut [T]) {
    if list.len() <= 1 {
        return;
    }
    let pivot = list[list.len() / 2].clone();
    let j = partition(list, pivot);
    _quick_sort(&mut list[..j]);
    _quick_sort(&mut list[j + 1..]);
}

fn partition<T: Ord>(list: &mut [T], pivot: T) -> usize {
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
            return j;
        }
        list.swap(i, j);
    }
}

pub fn _quick_selection<T: Ord + Clone>(list: &mut [T], k: usize) -> &T {
    if list.len() == 1 {
        return &list[0];
    }
    let pivot = list[list.len() / 2].clone();
    let j = partition(list, pivot);
    if j == k {
        return &list[j];
    }
    if j > k {
        return _quick_selection(&mut list[..j], k);
    }
    _quick_selection(&mut list[j + 1..], k - j - 1)
}
