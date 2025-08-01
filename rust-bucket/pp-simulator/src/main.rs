fn main() {
    let my_part = gen_part(String::from("blargy_pants"));
    let id = Identity(
        37,
        String::from("my_blargy_pants"),
        String::from("abc123qwerty999"),
    );

    let another_part = Part { id: 43, ..my_part };

    println!(
        "{} ({}) sn: {}",
        another_part.name, another_part.id, another_part.serial_number
    );
    println!("{}", id.0);
    println!("{}", id.1);
    println!("{}", id.2);
}

fn gen_part(name: String) -> Part {
    Part {
        id: 42,
        serial_number: String::from("abc123qwerty999"),
        name,
    }
}
struct Part {
    id: u32,
    serial_number: String,
    name: String,
}

struct Identity(u32, String, String);
