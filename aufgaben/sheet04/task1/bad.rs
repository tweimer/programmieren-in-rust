/// print if they are both
fn main() {
    for i in 1..21 {
        if is_happy_prime(i) {
            println!("{} is a happy prime!", i);
        }
    }
}

/// is it happy and prime?
fn is_happy_prime(n: u32) -> bool {
	is_happy(n) && is_prime(n)
}

/// Is it a happy number? https://en.wikipedia.org/wiki/Happy_number
fn is_happy(mut number: u32) -> bool {
    while number > 1 {
		number = {
		    let mut sum = 0;
		    while number > 0 {
				let digit = number % 10;
		        sum += digit * digit;
		        number /= 10;
		    }
			sum
        };

        // We ended up in a cycle -> not happy
        if number == 4 {
            return false;
        }
    }

    true
}

/// is it prime?
fn is_prime(n: u32) -> bool {
    if n == 1 {
        false
    }
	else {
		for divisor in 2..n {
		    if n % divisor == 0 {
		        return false;
		    }
		}
		true
	}
}
