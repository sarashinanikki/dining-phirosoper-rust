use std::thread;
use std::time::Duration;

struct Philosoper {
    name: String,
}

impl Philosoper {
    fn new(name: &str) -> Philosoper {
        Philosoper {
            name: name.to_string(),
        }
    }

    fn eat(&self) {
        println!("{} is eating!", self.name);
        thread::sleep(Duration::from_micros(1000));
        println!("{} is done eating!", self.name);
    }
}

fn main() {
    let philosopers = vec![
        Philosoper::new("Socrates"),
        Philosoper::new("Plato"),
        Philosoper::new("Aristoteles"),
        Philosoper::new("Anaximandros"),
        Philosoper::new("Epikouros"),
    ];

    for p in philosopers {
        p.eat();
    }
}
