pub fn lifetime() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("longest: {result}");

    let other1 = String::from("long string is long");

    {
        let other2 = String::from("shorty");
        let result = longest(other1.as_str(), other2.as_str());
    }
    println!("longest: {result}");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
