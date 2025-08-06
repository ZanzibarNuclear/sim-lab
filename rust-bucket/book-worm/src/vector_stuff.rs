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

    let mode: Vec<i32> = histogram
        .iter()
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

struct PigConverter {
    word: String,
    // start_punc: String,
    // end_punc: String,
}

impl PigConverter {
    pub fn new(word: &str) -> PigConverter {
        if word.is_empty() {
            panic!("I cannot deal with words that have no letters.")
        }

        // let mut index = 0;
        // let starts_at = loop {
        //     if &word.len() <= index + 1 {
        //         break index;
        //     }
        //     let next_char = &word[index..(index + 1)];
        // }
        //     if next_char == {
        //     }
        // }

        PigConverter {
            word: word.to_string(),
        }
    }
}

pub fn to_pig_latin(expression: &str) -> String {
    expression
        .split_whitespace()
        .map(|word| {
            let (core, suffix, punctuation) = if word.ends_with(|c: char| c.is_ascii_punctuation()) {
                let core_word = &word[..word.len() - 1];
                let punc = &word[word.len() - 1..];
                if "aeiouAEIOU".contains(&core_word[..1]) {
                    (core_word, "-hay".to_string(), punc)
                } else {
                    (&core_word[1..], format!("-{}ay", &core_word[..1].to_lowercase()), punc)
                }
            } else {
                if "aeiouAEIOU".contains(&word[..1]) {
                    (word, "-hay".to_string(), "")
                } else {
                    (&word[1..], format!("-{}ay", &word[..1]), "")
                }
            };
            format!("{}{}{}", core, suffix, punctuation)
        })
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn en_to_pl(expression: &str) -> String {
    print!("{expression}");

    // split expression into words
    let words = expression.split_whitespace().collect::<Vec<&str>>();
    println!("Words: {:?}", words);

    // do something about punctuation
    let mut buffer = String::from("");
    for word in words {
        // hold on to punctuation, use slice that excludes

        if word.ends_with('.') || word.ends_with(',') || word.ends_with('!') || word.ends_with('?')
        {
            let found_punc = true;
            let the_punc = &word[(word.len() - 1)..];
        }

        // check for starting with vowel
        if word.starts_with('a')
            || word.starts_with('e')
            || word.starts_with('i')
            || word.starts_with('o')
            || word.starts_with('u')
        {
            // append word + "hay" to output
            buffer.push_str(word);
            buffer.push_str("-hay");
        } else {
            // otherwise, take first character ('c') - append the rest + 'c' + "ay"
            buffer.push_str(&word[1..]);
            buffer.push_str("-");
            buffer.push_str(&word[..1]);
            buffer.push_str("ay");
        }
        buffer.push_str(" ");
        // reattach punctuation as needed
    }
    buffer
}
