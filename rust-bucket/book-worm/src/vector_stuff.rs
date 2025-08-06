use std::collections::HashMap;

pub fn find_median(values: &Vec<i32>) -> f32 {
    let mut scratch = values.clone();
    scratch.sort();

    let how_many = scratch.len();
    let mid_index = how_many / 2;
    let median1 = scratch.get(mid_index).expect("should be an integer");

    if how_many % 2 == 0 {
        let median2 = scratch
            .get(mid_index - 1)
            .expect("should be another integer");
        let answer: f32 = (*median1 as f32 + *median2 as f32) / 2.0;
        answer
    } else {
        *median1 as f32
    }
}

pub fn find_mode(values: &Vec<i32>) {
    let value_count = values.len();
    println!("Determining mode of: {} values", value_count);

    let mut high_water = 0;
    let mut histogram: HashMap<i32, i32> = HashMap::with_capacity(value_count);

    for value in values {
        let count = histogram.entry(*value).or_insert(0);
        *count += 1;
        if *count > high_water {
            high_water = *count;
        }
    }

    println!("A la mode: {histogram:?}");

    let mode: Vec<i32> = histogram.iter()
        .filter(|&(_k, &v)| v == high_water)
        .map(|(&k, _v)| k)
        .collect();

    println!("Mode: {:?}", mode);
}

fn ai_find_mode() {
    let mut items: HashMap<String, i32> = HashMap::new();
    items.insert("apple".to_string(), 42);
    items.insert("banana".to_string(), 10);
    items.insert("cherry".to_string(), 42);
    items.insert("date".to_string(), 20);

    // Filter entries where value is 42, collect into a vector of references
    let filtered_entries: Vec<(&String, &i32)> = items.iter().filter(|&(_k, v)| *v == 42).collect();

    // Extract keys from filtered entries into a vector
    let filtered_keys: Vec<&String> = filtered_entries.iter().map(|&(k, _v)| k).collect();

    // Print results
    println!("Filtered entries: {:?}", filtered_entries);
    println!("Filtered keys: {:?}", filtered_keys);
}
