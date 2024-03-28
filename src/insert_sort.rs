pub fn insertion_sort(arr: &mut [i32]) {
    let mut i = 1;
    while i < arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
        i += 1;
    }
}