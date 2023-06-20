pub fn median(list: &Vec<i32>) -> i32 {
    let mut list: Vec<i32> = list.clone();
    list.sort();

    let n = list.len();

    if n % 2 == 1 {
        list[(n - 1) / 2]
    } else {
        (list[n/2 - 1] + list[n/2]) / 2
    }
}

pub fn mode(list: &Vec<i32>) -> i32 {
    use std::collections::HashMap;
    
    let mut map: HashMap<i32, u32> = HashMap::new();

    for value in list {
        let count = map.entry(*value).or_insert(0);
        *count += 1;
    }

    struct ValueCountPair {
        value: i32,
        count: u32
    }

    let mut current_mode = ValueCountPair {value: 0, count: 0};

    for (value, count) in map {
        if (current_mode.count < count) ||
        (current_mode.count == count && current_mode.value > value) {
            current_mode = ValueCountPair {value, count}
        }
    }

    current_mode.value
}