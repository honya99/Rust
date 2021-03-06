/// In place counting sort for collections of u32
/// O(n + maxval) in time, where maxval is the biggest value an input can possibly take
/// O(maxval) in memory
/// u32 is chosen arbitrarly, a counting sort probably should'nt be used on data that requires bigger types.
pub fn counting_sort(arr: &mut [u32], maxval: usize) {
    let mut occurences: Vec<usize> = vec![0; maxval + 1]; 

    for &data in arr.iter() {
        occurences[data as usize] += 1;
    }

    let mut i = 0;
    for (data, &number) in occurences.iter().enumerate() {
        for _ in 0..number {
            arr[i] = data as u32;
            i += 1;
        }
    }
}

use std::ops::AddAssign;
/// Generic implementation of a counting sort for all usigned types
pub fn generic_counting_sort<T: Into<u64> + From<u8> + AddAssign + Copy>(arr: &mut [T], maxval: usize) {
    let mut occurences: Vec<usize> = vec![0; maxval + 1]; 

    for &data in arr.iter() {
        occurences[data.into() as usize] += 1;
    }

    let mut i = 0;
    // current data point, necessary to be type-safe
    let mut data = T::from(0);

    for &number in occurences.iter() {
        for _ in 0..number {
            arr[i] = data;
            i += 1;
        }
        data += T::from(1);
    }
}
