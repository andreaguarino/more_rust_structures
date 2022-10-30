pub fn quick_sort<T: Ord + Clone>(list: &mut Vec<T>) {
    if list.len() <= 1 {
        return;
    }
    let low = 0;
    let high = list.len() - 1;
    _quick_sort(list, low, high);
}

pub fn quick_selection<T: Ord + Clone>(list: &mut Vec<T>, k: usize) -> Option<&T> {
    if list.len() == 0 {
        return None;
    }
    let low = 0;
    let high = list.len() - 1;
    _quick_selection(list, k, low, high)
}

fn _quick_sort<T: Ord + Clone>(list: &mut [T], low: usize, high: usize) {
    if low >= high {
        return;
    }
    let p = partition(list, low, high);
    _quick_sort(list, low, p);
    _quick_sort(list, p + 1, high);
}

fn partition<T: Ord + Clone>(list: &mut [T], low: usize, high: usize) -> usize {
    let pivot = list[(high + low) / 2].clone();
    let mut i = low as i32 - 1;
    let mut j = high as i32 + 1;
    loop {
        i += 1;
        while list[i as usize] < pivot {
            i += 1;
        }
        j -= 1;
        while list[j as usize] > pivot {
            j -= 1;
        }
        if i >= j {
            return j as usize;
        }
        list.swap(i as usize, j as usize);
    }
}

pub fn _quick_selection<T: Ord + Clone>(
    list: &mut [T],
    k: usize,
    low: usize,
    high: usize,
) -> Option<&T> {
    if k > high {
        return None;
    }
    let j = partition(list, low, high);
    if j == k {
        return Some(&list[j]);
    }
    if j > k {
        return _quick_selection(list, k, low, j);
    }
    _quick_selection(list, k, j + 1, high)
}
