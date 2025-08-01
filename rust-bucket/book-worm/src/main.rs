#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {rect1:?} with an area of {}", area(&rect1));
}

fn area(dim: &Rectangle) -> u32 {
    dim.width * dim.height
}
