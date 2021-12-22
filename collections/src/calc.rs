use std::vec::Vec;
use std::collections::HashMap;

pub fn mean(v: &Vec<i32>) -> i32 {
    let mut sum = 0;

    for elem in v {
        sum += elem
    }

    sum / v.len() as i32
}

pub fn median(v: &Vec<i32>) -> i32 {
    v[v.len() / 2]
}

pub fn mode(v: &Vec<i32>) -> i32 {
    let mut counts: HashMap<i32, i32> = HashMap::new();

    for elem in v {
        if let Some(current_elem_count) = &counts.get(elem) {
            counts.insert(*elem, *current_elem_count + 1);
        } else {
            counts.insert(*elem, 1);
        }
    }

    let mut mode_val = 0;
    let mut mode_counts = 0;

    for (key, val) in counts {
        println!("{} , {}", key, val);
        if val > mode_counts {
            mode_val = key;
            mode_counts = val;
        }
    }

    mode_val
}