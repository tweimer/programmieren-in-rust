//! Aufgabe 2.3

fn main() {
    let count = count("peter", 'e');
    println!("{}", count);
}


fn count(text: &str, c: char) -> u64 {
    let mut count = 0;
    let chars = text.chars();
    for c1 in chars {
        if c1 == c {
            count+=1;
        }
    }

    count
}
