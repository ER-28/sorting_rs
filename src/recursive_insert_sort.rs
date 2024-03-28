pub fn recursive_insert_sort(arr: &mut [i32], n: usize) -> &mut [i32] {
    if n < arr.len() {
        let mut i = n;
        while i > 0 && arr[i - 1] > arr[i] {
            arr.swap(i - 1, i);
            i -= 1;
        }
        recursive_insert_sort(arr, n + 1);
    }
    arr
}