pub struct BlargyStuff {
    pub name: String,
    pub description: String,
    level: u8,
}

impl BlargyStuff {
    pub fn new(name: &str) -> Self {
        BlargyStuff {
            name: name.to_string(),
            description: String::from("n/a"),
            level: 1,
        }
    }
    pub fn level_up(&mut self) {
        self.level += 1;
    }
    pub fn name_tag(&self) {
        println!("Hello, my name is {}!", self.name);
        if self.level > 5 {
            println!("Bow down to me if you want to live.")
        }
    }

    pub fn read_at(&self, index: usize) -> i32 {
        let v = vec![1, 2, 3];
        v[index]
    }

    pub fn panic(&self) {
        panic!("Oh no!")
    }
}
