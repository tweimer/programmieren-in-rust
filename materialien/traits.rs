trait Speak {
    fn speak(&self);
}

struct Cat;

impl Speak for Cat {
    fn speak(&self) {
        println!("meow");
    }
}

struct Pokemon { name: String }

impl Speak for Pokemon {
    fn speak(&self) {
        println!("{}", self.name);
    }
}


fn main() {
    let cat = Cat;
    cat.speak();
    
    
    let poki = Pokemon {
        name: "Peter".into(),
    };
    poki.speak();
}

