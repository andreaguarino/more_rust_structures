fn selection_sort<T: Ord>(list: &mut Vec<T>) {
    for i in 0..list.len() {
        for j in i + 1..list.len() {
            if list[j] < list[i] {
                list.swap(i, j)
            }
        }
    }
}

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
        let mut list = vec![5, 1, 42, 0];
        selection_sort(&mut list);
        assert_eq!(list, vec![0, 1, 5, 42]);
    }

    #[test]
    fn insertion_sort_test() {
        let mut list = vec![5, 1, 42, 0];
        insertion_sort(&mut list);
        assert_eq!(list, vec![0, 1, 5, 42]);
    }
}
