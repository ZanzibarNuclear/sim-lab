use core::num;

pub fn fun_w_generics() {
    let number_list = vec![34, 50, 25, 100_000_000, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {result}.");

    let char_list = vec!['y', 'あ', 'a', 'ん', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {result}");

    make_good_points();
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

struct Pointy<X, Y> {
    x: X,
    y: Y,
}

impl<X1, Y1> Pointy<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Pointy<X2, Y2>) -> Pointy<X1, Y2> {
        Pointy {
            x: self.x,
            y: other.y,
        }
    }
}

fn make_good_points() {
    let on_screen = Point { x: 15u32, y: 5u32 };
    let on_plot = Point { x: -10, y: 42 };

    println!("p.x = {}", on_screen.x());
    println!("p.x = {}", on_plot.x());

    let p1 = Pointy { x: 5, y: 10.4 };
    let p2 = Pointy { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
