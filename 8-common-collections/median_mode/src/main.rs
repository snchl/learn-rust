use std::collections::HashMap;

fn main() {
    let mut v = vec![1, 6, 12, 8, 92, 5, 6, 12, 12, 8, 234, 456, 234, 7655, 234];

    // Determine median of vector `v`
    v.sort();

    let v_is_even = v.len() % 2 == 0;

    if v_is_even {
        let v_center_index = v.len() / 2;
        let median = (v[v_center_index] + v[v_center_index + 1]) / 2;

        println!("Median of `v` is {}", median);
    } else {
        let v_len = v.len();
        let v_len_float = v_len as f64;
        let median_index_float= (v_len_float / 2.0) + 0.5;
        let median_index = median_index_float as usize;

        println!("Median of `v` is {}", v[median_index]);
    }

    // Determine mode value of vector `v`
    let mut hash: HashMap<i32, i32> = HashMap::new();

    let mut mode_value = v[0];
    let mut mode_count = 0;

    for value in &v {
        let count = hash.entry(*value).or_insert(0);

        *count += 1;

        if *count > mode_count {
           mode_count = *count;
           mode_value = *value; 
        }
    }

    println!("Mode value is {} with {} occurence", mode_value, mode_count);
}
