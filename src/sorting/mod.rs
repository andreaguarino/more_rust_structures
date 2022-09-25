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

#[cfg(test)]
mod tests {
    use crate::sorting::{insertion_sort, selection_sort};

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
