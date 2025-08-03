#![allow(unused)]

mod blargy;

fn main() {
    string_theory();
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
