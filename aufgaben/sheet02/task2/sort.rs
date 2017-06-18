fn main() {
    let mut arr = [61, 21, 27, 79, 57, 60, 46, 42, 27, 92, 66, 26];

    sort(&mut arr);
    println!("{:?}", arr);
}

// Sorts Array in-place using Selectionsort
fn sort(arr: &mut [u64]) {
    let len = arr.len();
    for start_idx in 0..len {
        let mut min_idx = start_idx;

        for idx in start_idx..len {
            if arr[idx] < arr[min_idx] {
                min_idx = idx;
            }
        }

        // Swap start_idx and min_idx
        let val = arr[min_idx];
        arr[min_idx] = arr[start_idx];
        arr[start_idx] = val;
    }
}


#[test]
fn sort_array() {
    let mut arr =  [
        61, 21, 27, 79, 57, 60, 46, 92, 66, 26, 37, 15, 29, 70, 30, 55, 62, 81,
        84, 35, 34, 52, 98, 50, 39, 42, 41, 24, 28, 64, 95, 47, 43, 23, 14, 71,
        78, 86, 51, 20, 9, 1, 18, 17, 94, 33, 3, 91, 65, 2, 38, 59, 96, 8, 83,
        19, 90, 63, 16, 58, 68, 48
    ];
    sort(&mut arr);
    assert_eq!(&arr as &[u64], &[
        1u64, 2, 3, 8, 9, 14, 15, 16, 17, 18, 19, 20, 21, 23, 24, 26, 27, 28, 29,
        30, 33, 34, 35, 37, 38, 39, 41, 42, 43, 46, 47, 48, 50, 51, 52, 55, 57,
        58, 59, 60, 61, 62, 63, 64, 65, 66, 68, 70, 71, 78, 79, 81, 83, 84, 86,
        90, 91, 92, 94, 95, 96, 98,
    ] as &[u64]);
}
