use rand::seq::{IndexedRandom, SliceRandom};
use crate::is_sorted::is_sorted;


pub fn bozo_sort(arr: &mut [i32]) {
    while !is_sorted(arr) {
        let (i, j) = {
            let mut rng = rand::thread_rng();
            let i = *arr.choose(&mut rng).unwrap();
            let j = *arr.choose(&mut rng).unwrap();
            (i, j)
        };
        arr.swap(i as usize, j as usize);
    }
}