pub struct InsertionSort;

impl InsertionSort {
    fn sort<T>(list: &mut [T])
    where
        T: PartialOrd + Copy
    {
        for i in 1..list.len() {
            let x = list[i];
            let mut j = (i - 1) as isize;
            while j >= 0 && list[j as usize] > x {
                list.swap((j + 1) as usize, j as usize);
                j -= 1;
            }
            list[(j + 1) as usize] = x;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::InsertionSort;

    #[test]
    fn test_insertion_sort() {
        let mut some_list: [i32; 0] = [];
        InsertionSort::sort(&mut some_list);
        assert_eq!(some_list, []);

        let mut some_list = [2];
        InsertionSort::sort(&mut some_list);
        assert_eq!(some_list, [2]);
        
        let mut some_list = [1, 4, 7, 2, 9, 0, 3];
        InsertionSort::sort(&mut some_list);
        assert_eq!(some_list, [0, 1, 2, 3, 4, 7, 9]);
    }
}   