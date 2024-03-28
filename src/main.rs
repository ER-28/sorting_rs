mod bubble_sort;
mod insert_sort;
mod recursive_insert_sort;
mod bogo_sort;
mod bozo_sort;
mod is_sorted;


fn main() {

    let mut arr = [64, 34, 25, 12, 22, 11, 90];
    let mut arr2 = [64, 34, 25, 12, 22, 11, 90];

    println!("Unorted array: {:?}", arr);

    bubble_sort::bubble_sort(&mut arr2);
    println!(" Sorted array: {:?}", arr2);
}
