pub fn reorder_and_pad(slice: &mut [i32], is_group_one: fn(i32) -> bool, pad_value: i32) -> usize {
    let mut group_one_count = 0;

    // Reorder elements based on `is_group_one`
    let mut temp = Vec::with_capacity(slice.len());
    for &x in slice.iter() {
        if is_group_one(x) {
            temp.push(x);
            group_one_count += 1;
        }
    }
    for &x in slice.iter() {
        if !is_group_one(x) {
            temp.push(x);
        }
    }

    // Copy reordered elements back to the slice
    for (i, &x) in temp.iter().enumerate() {
        slice[i] = x;
    }

    // Pad the remaining slice with `pad_value`
    for i in group_one_count + (slice.len() - temp.len())..slice.len() {
        slice[i] = pad_value;
    }

    group_one_count
}
