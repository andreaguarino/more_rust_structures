mod quick_sort;

///     ## Advantages and Disadvantages
///     - The algorithm performs sorting in place and is online.
///     - It is not stable in its basic form and requires additional space or specific assumptions on the type of list
///       being sorted (such as it being a linked list).
///     - Compared to other quadratic comparison-based algorithms, such as <see cref="InsertionSort"/>, it is generally
///       simpler but requires in average an higher number of comparisons, therefore yielding worse performance.
///     - Compared to linearithmic comparison-based algorithms, such as <see cref="HeapSort"/>, it is way simpler to
///       understand and predict in exact number of operations executed. However, the performance is sensibly worse.
///     - Compared to non-comparison-based algorithms, such as counting sort, it doesn't require any assumption on the
///       type or values of the items in the input, the only requirement being their total comparability and the
///       comparison behaving according to total order operators rules.
///     ## Algorithm
///     - This sorting algorithm split the list L being sorted in two parts: the sorted part, located at the beginning
///       of the list (`L[..i]`), and the unsorted part, located at the end of the list (`L[i..]`).
///     - At the beginning the sorted part is empty (i.e. length 0) and the unsorted part covers the entire list (i.e.
///       length n).
///     - The algorithm runs n iterations, where n is the number of items in the list.
///     - At the beginning of iteration `i`, the sorted sub-list is `L[..i]` and the unsorted sub-list is `L[i..]`.
///     - The unsorted sub-list `L[i..]` is scanned linearly, looking for the index `j`, between `i` and `n - 1`, of the item
///       of `L[i..]` with minimum value.
///     - `L[i]` is swapped with `L[j]` and the iteration `i` terminates.
///     - Now `L[..(i + 1)]` is the new sorted sub-list, and `L[(i + 1)..]` is the new unsorted sub-list.
///     ## Complexity
///     - Each of the n iterations runs `n - i - 1` comparisons, to identify the index of the item with the minimum value
///       in the sub-list `L[i..]`.
///     - The total number of comparisons, over the n iterations, is around `n * n / 2`.
///     - Therefore, Time Complexity is `O(n^2)` and Space Complexity is `O(1)`, since the algorithm runs in place.
fn selection_sort<T: Ord>(list: &mut Vec<T>) {
    for i in 0..list.len() {
        for j in i + 1..list.len() {
            if list[j] < list[i] {
                list.swap(i, j)
            }
        }
    }
}

///     ## Algorithm
///     - This sorting algorithm split the list L being sorted in two parts: the sorted part, located at the beginning
///       of the list (`L[..i]`), and the unsorted part, located at the end of the list (`L[i..]`).
///       <br/>
///     - At the beginning the sorted part is empty (i.e. length 0) and the unsorted part covers the entire list (i.e.
///       length n).
///     - The algorithm runs n - 1 1-based iterations, where n is the number of items in the list.
///     - At the beginning of iteration i, the sorted sub-list is `L[..i]` and the unsorted sub-list is `L[i..]`.
///     - The first item `L[i]`, of the unsorted sub-list `L[i..]`, is compared against its predecessor, `L[i - 1]`.
///     - If `L[i]` is smaller than `L[i - 1]`, the two items are swapped and the new `L[i - 1]` is compared with `L[i - 2]`.
///       Comparisons and swapping continues until the predecessor is not bigger than its successor, potentially until
///       the head of the list is reached.
///     - When a `L[j]` is found, which is not strictly smaller than `L[j - 1]`, `L[.. (i + 1)]` is sorted, and the iteration
///       i can terminate.
///     ## Complexity
///     - Each of the n - 1 iterations runs at most i - 1 comparisons, if it has to swap all the way up to the head of
///       the list.
///     - The total number of comparisons, over the n iterations, is around `n * n / 2`.
///     - Therefore, Time Complexity is `O(n^2)` and Space Complexity is `O(1)`, since the algorithm runs in place and
///       hence only requires additional constant space to perform the sorting.
fn insertion_sort<T: Ord>(list: &mut Vec<T>) {
    for i in 0..list.len() {
        for j in (1..i + 1).rev() {
            if list[j] < list[j - 1] {
                list.swap(j, j - 1)
            } else {
                break;
            }
        }
    }
}

///     ## ADVANTAGES AND DISADVANTAGE
///      - Compared to [selection_sort] and [insertion_sort], it has asymptotically better
///       runtime, linearithmic instead of quadratic.
///     - Compared to [quick_sort], it has **better worst-case performance**: linearithmic, instead of
///       quadratic.
///     - In general [merge_sort] does a number of comparisons which is independent from the
///       input, whereas the number of comparisons in [quick_sort] depends on the input and in particular on
///       the choice of the pivot.
///     - Unlike [selection_sort], [quick_sort] and shell_sort, and like
///       [insertion_sort] it is a **stable** sorting algorithm, so it preserves the order in the input
///       of items with the same key.
///     - A disadvantage over many other sorting algorithms is that it is **not
///       in place**, as it requires additional `O(n)` space to perform the sorting.
///     - An advantage over many other sorting algorithms, and in particular over [quick_sort], is that it
///       is *easily parallelizable*.
///     - The reason is that [merge_sort] is based on a "divide and conquer" design, where the partition of
///       the problem in sub-problems is done in `O(1)` time (just split the array in two halves) while the combine phase
///       (i.e. the merge) is more complex, taking `O(n)` time. In `quick_sort` it is rather the opposite: the
///       partition is the `O(n)` operation, while the combine comes for free.
///     - Another advantage of [merge_sort] over other comparison-based algorithms is that it performs an
///       **optimal number of comparisons**: `n * log(n)`. No other comparison-based algorithm can do better. That,
///       however, doesn't mean that [merge_sort] performs better than any other comparison-based algorithm:
///       while [quick_sort] performs more comparisons in average (~ 39% more, in practice), it also does
///       sorting in place and does many less operations, resulting in better performance.
///     COMPLEXITY
///     - Every recursive call splits the input into two halves of roughly equal size.
///     - Therefore, the depth of recursion is the logarithm of n in base 2.
///     - Each recursive call performs an amount of work which is linear in the size of its input, and also uses a
///       linear amount of additional space in T, which, however, is instantiated once, at top level.
///     - Therefore, Time Complexity is O(n * log(n)) and Space Complexity is O(n).
fn merge_sort<T: Ord + Copy>(list: &mut Vec<T>) {
    let mut aux = list.clone();
    _merge_sort(list.as_mut_slice(), aux.as_mut_slice());
}

fn _merge_sort<T: Ord + Copy>(list: &mut [T], aux: &mut [T]) {
    let len = list.len();
    if len <= 1 {
        return;
    }
    _merge_sort(&mut list[..len / 2], &mut aux[..len / 2]);
    _merge_sort(&mut list[len / 2..], &mut aux[len / 2..]);
    merge(list, aux);
}

fn merge<T: Ord + Copy>(list: &mut [T], aux: &mut [T]) {
    let len = list.len();
    let mut i = 0;
    let mut j = len / 2;
    let mut k = 0;
    while i < len / 2 && j < len {
        if list[i] <= list[j] {
            aux[k] = list[i];
            i += 1;
            k += 1;
        } else {
            aux[k] = list[j];
            j += 1;
            k += 1;
        }
    }
    while i < len / 2 {
        aux[k] = list[i];
        i += 1;
        k += 1;
    }
    while j < len {
        aux[k] = list[j];
        j += 1;
        k += 1;
    }
    list.copy_from_slice(&aux);
}

#[cfg(test)]
mod tests {
    use super::{
        insertion_sort, merge_sort,
        quick_sort::{quick_selection, quick_sort},
        selection_sort,
    };

    #[test]
    fn selection_sort_test() {
        sorting_test(selection_sort);
        sorting_test_str(insertion_sort);
    }

    #[test]
    fn insertion_sort_test() {
        sorting_test(insertion_sort);
        sorting_test_str(insertion_sort);
    }

    #[test]
    fn merge_sort_test() {
        sorting_test(merge_sort);
        sorting_test_str(merge_sort);
    }

    #[test]
    fn quick_sort_test() {
        sorting_test(quick_sort);
        sorting_test_str(quick_sort);
    }

    #[test]
    fn quick_selection_test() {
        let mut list = vec![4, 7, 11, 0, 3, 5, 42, 12];
        assert_eq!(*quick_selection(&mut list, 0).unwrap(), 0);
        assert_eq!(*quick_selection(&mut list, 1).unwrap(), 3);
        assert_eq!(*quick_selection(&mut list, 2).unwrap(), 4);
        assert_eq!(*quick_selection(&mut list, 3).unwrap(), 5);
        assert_eq!(*quick_selection(&mut list, 4).unwrap(), 7);
        assert_eq!(*quick_selection(&mut list, 5).unwrap(), 11);
        assert_eq!(*quick_selection(&mut list, 6).unwrap(), 12);
        assert_eq!(*quick_selection(&mut list, 7).unwrap(), 42);
        assert_eq!(quick_selection(&mut list, 8), None);

        let mut list = vec![4, 7, 11, 0, 3, 3, 5, 42, 12];
        assert_eq!(*quick_selection(&mut list, 4).unwrap(), 5);

        let mut list = vec![0, 0, 0, 0, 0];
        assert_eq!(*quick_selection(&mut list, 4).unwrap(), 0);

        let mut list: Vec<i32> = vec![];
        assert_eq!(quick_selection(&mut list, 4), None);
    }

    fn sorting_test<F>(sorting_function: F)
    where
        F: (Fn(&mut Vec<i32>) -> ()),
    {
        let mut list = vec![5, 1, 42, 0];
        sorting_function(&mut list);
        assert_eq!(list, vec![0, 1, 5, 42]);

        let mut empty_array: Vec<i32> = vec![];
        sorting_function(&mut empty_array);
        assert_eq!(empty_array, empty_array);

        let mut singleton_array: Vec<i32> = vec![42];
        sorting_function(&mut singleton_array);
        assert_eq!(singleton_array, vec![42]);

        let mut longer_list = vec![4, 7, 11, 0, 3, 5, 42, 12];
        sorting_function(&mut longer_list);
        assert_eq!(longer_list, vec![0, 3, 4, 5, 7, 11, 12, 42]);

        let mut longer_list = vec![4, 7, 11, 7, 7, 7, 3, 5, 42, 12];
        sorting_function(&mut longer_list);
        assert_eq!(longer_list, vec![3, 4, 5, 7, 7, 7, 7, 11, 12, 42]);
    }

    fn sorting_test_str<F>(sorting_function: F)
    where
        F: (Fn(&mut Vec<char>) -> ()),
    {
        let mut list = vec!['d', 'b', 'c', 'a'];
        sorting_function(&mut list);
        assert_eq!(list, vec!['a', 'b', 'c', 'd']);
    }
}
