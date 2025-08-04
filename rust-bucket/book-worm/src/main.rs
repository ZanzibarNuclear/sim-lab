#![allow(unused)]

mod blargy;

fn main() {
    let ans1 = find_median(&[1, 2, 3]);
    let ans2 = find_median(&[2, 3, 4, 4, 5, 6, 6, 6, 7, 8, 9, 10]);

    println!("answers: {} {}", ans1, ans2);
}

/*
Practice exercises:

1. Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.

2. Convert strings to pig latin. The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay. Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!

3. Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company; for example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
*/

fn find_median(values: &[i32]) -> i32 {
    let mut sorted: Vec<i32> = Vec::new();
    sorted.copy_from_slice(&values);
    sorted.sort();
    let how_many = sorted.len();
    let mid = how_many / 2;
    if how_many % 2 == 0 {
        (sorted.get(mid).unwrap() + sorted.get(mid - 1).unwrap()) / 2
    } else {
        sorted.get(mid).unwrap() / 1
    }
}

fn hashery() {
    use std::collections::HashMap;

    let text = "hello world wonderful wonderful world hello hello am i right";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}

#[derive(Debug)]
struct SpaceStuff {
    label: String,
}

fn string_theory() {
    let stringy = "stringy";

    let string = SpaceStuff {
        label: stringy.to_string(),
    };
    let dust = SpaceStuff {
        label: "dusty".to_string(),
    };
    let space = SpaceStuff {
        label: "spacey".to_string(),
    };
    let moon = SpaceStuff {
        label: "月的".to_string(),
    };

    let space_v = vec![string, dust, space, moon];
    for i in &space_v {
        println!("This is rather {:?}.", i);
    }

    println!(
        "Come again. How is the first one? {:?}",
        space_v.get(0).unwrap().label
    );
}

fn respect_your_elders() {
    let mut my_blargy = blargy::BlargyStuff::new("Bubba");
    my_blargy.level_up();
    my_blargy.level_up();
    my_blargy.level_up();
    my_blargy.level_up();
    my_blargy.level_up();
    my_blargy.name_tag();
}

fn play_with_vectors() {
    let mut v = vec![1, 2, 3];

    v.push(9);
    v.push(8);
    v.push(7);

    println!("{:?}", v);
}

fn vector_fun() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}
