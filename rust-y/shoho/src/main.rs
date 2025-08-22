use shoho::Circle;

fn main() {
    println!("Welcome to the Shoho! These are the basics.");

    let round_friend = Circle::new(1.0);

    println!("The area of my round friend is {}", round_friend.area());
}
