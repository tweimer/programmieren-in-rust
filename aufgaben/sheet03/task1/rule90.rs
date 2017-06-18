//! Task 3.1: Rule 90
const NUM_ITERATIONS: u64 = 20;

fn main() {
    fn pr(state: &[bool]) {
        for &cell in state {
            print!("{}", if cell { "\u{2588}\u{2588}" } else { "  " });
            //print!("{}", if cell { "1" } else { "0" });
        }
        println!("");
    }


    let mut v = read_input();
    for _ in 1..NUM_ITERATIONS {
        pr(&v);
        v = next_step(&v);
    }
    pr(&v);    
}

/// Reads a valid initial configuration for our automaton from the terminal.
fn read_input() -> Vec<bool> {
    // This tries to read a string from the terminal, checks whether it's
    // valid (only contains 1's and 0's). If the user fails to input a correct
    // string, this routine will ask again until the user finally manages to
    // give us a correct string.
    //
    // You don't need to understand this routine yet; that's why I've written
    // it already ;-)
    //
    // You only need to use the `input` variable (of type `String`). You can
    // also assume that it only contains '0' and '1' chars.
    let input = {
        let mut buffer = String::new();

        loop {
            println!("Please give me the initial configuration (a string of '0' and '1'!):");
            buffer.clear();

            // `read_line` returns an error if the input isn't valid UTF8 or if
            // a strange IO error occured. We just panic in that case...
            std::io::stdin()
                .read_line(&mut buffer)
                .expect("something went seriously wrong :O");

            if buffer.trim().chars().all(|c| c == '1' || c == '0') {
                break;
            }
        }

        buffer.trim().to_string()
    };

    // a) Eingabe einlesen
    let mut vec = Vec::with_capacity(input.len());
    let chars = input.chars();
    for c in chars {
        vec.push(c == '1');
    }
    vec
}

// b) Zeitschritt simulieren
fn next_step(old: &[bool]) -> Vec<bool> {
    let end = old.len();
    let mut vec = Vec::with_capacity(end);

    for index in 0..end {
        let right_index = if index == end - 1 { 0 } else { index + 1 };
        let left_index = if index == 0 { end - 1 } else { index - 1 };

        // XOR
        vec.push(old[left_index] ^ old[right_index]);
    }
    vec
    
}

#[test]
fn rule90_rules() {
    assert_eq!(next_step(&[false, false, false]), vec![false, false, false]);
    assert_eq!(next_step(&[ true, false, false]), vec![false,  true,  true]);
    assert_eq!(next_step(&[ true,  true, false]), vec![ true,  true, false]);
    assert_eq!(next_step(&[ true,  true,  true]), vec![false, false, false]);
}
