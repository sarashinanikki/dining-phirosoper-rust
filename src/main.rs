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
        println!("{} is done eating!", self.name);
    }
}

fn main() {
    let Philosopers = vec![
        Philosoper::new("Socrates");
        Philosoper::new("Plato");
        Philosoper::new("Aristoteles");
        Philosoper::new("Anaximandros");
        Philosoper::new("Epikouros");
    ];

    for p in Philosopers {
        p.eat();
    }
}
