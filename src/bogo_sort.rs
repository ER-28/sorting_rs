use rand::seq::SliceRandom;
use crate::is_sorted::is_sorted;

pub fn bogo_sort(arr: &mut [i32]) {
    while !is_sorted(arr) {
        arr.shuffle(&mut rand::thread_rng());
    }
}