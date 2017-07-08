struct Fib {a: u64, b: u64 }

impl Fib {
    fn new() -> Self {
        Self{a:0, b:1}
    }
}

impl Iterator for Fib {
    type Item = u64;
    fn next (&mut self) -> Option<Self::Item> {
        let c = self.a + self.b;
        self.a = self.b;
        self.b = c;
        Some(c)
    }
}

fn main() {
    let fib = Fib::new();
    for i in fib.take(20) {
        println!("{}", i);
    }
}
