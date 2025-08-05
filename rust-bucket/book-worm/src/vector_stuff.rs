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
