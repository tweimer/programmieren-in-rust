/// print if they are both
fn main() {
    for iterationnumber in &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20] {
        let iterationnumber = *iterationnumber;
        if happy_prime(iterationnumber) {
            println!("{} is a happy prime!", iterationnumber);
        }
    }
}

/// is it happy and prime?
fn happy_prime(n: u32) -> bool {
	check_if_number_is_happy(n) && check_if_number_is_prime(n)
}

/// Is it a happy number? https://en.wikipedia.org/wiki/Happy_number
fn check_if_number_is_happy(mut number: u32) -> bool {
    while number > 1 {
        let mut tmp = 0;
        while number > 0 {
            tmp += (number %10) * (number%10);
            number /= 10;
        }
        number = tmp;

        // We ended up in a cycle -> not happy
        if number == 4 {
            return false;
        }
    }

    true
}

/// is it prime?
fn check_if_number_is_prime(n: u32) -> bool {
    if n == 1 || n == 2 {
        false
    }
	else {
		let mut teiler_gefunden = false;

		let mut teiler = 2u32;
		while teiler < n {
		    if n % teiler == 0 {
		        teiler_gefunden = true;
		    }
		    teiler = teiler + 1;
		}

		!teiler_gefunden
	}
}
