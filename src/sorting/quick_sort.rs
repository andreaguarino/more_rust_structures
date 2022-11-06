///     ## ADVANTAGES AND DISADVANTAGES
///     - Like [merge_sort], it belongs to the category of linearithmic comparison-based sorting algorithm
///       (when setup correctly, see the complexity analysis section below).
///     - Compared to [merge_sort], it is <b>in-place</b>, having `O(1)` Space Complexity.
///     - However, it is *not stable* and it's *not as easy parallelizable* as [merge_sort].
///     - While it is not optimal in the exact number of comparisons (it does in average 39% more comparisons than
///       [merge_sort], which is optimal on the number of comparisons), it does way less swapping and moving
///       operations, resulting in a visibly better performance, especially when cost of moving items is high.
///     - For that reason is tipically the <b>algorithm of choice when sorting primitive values or value types</b>,
///       such as struct instances, where cost of comparison is low and cost of swapping can be high, depending on the
///       size of the struct.
///     - When sorting reference types, such as class instances, [merge_sort] is sometimes preferred, since
///       swapping items in the list means swapping their references, which are of fixed and small size, and instead
///       the cost of comparison can be quite high, depending on the implementation used.
///     - Compared to most other comparison-based algorithms, a disadvantage of quicksort is that, for it to have
///       consistently good performances, it has to be randomized. In such setup, it is <b>not deterministic</b>.
///     COMPLEXITY
///     <br/>
///     - Worst-case Time Complexity is `O(n^2)` and Space Complexity is `O(1)`, since sorting happens entirely in place,
///       by repeated swapping.
///       <br/>
///     - Expected Time Complexity heavily depends on the choice of the shuffle strategy and
///       the partion strategy, which in turns depends on the choice of pivot.
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

/// Uses a Hoare partition strategy
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
