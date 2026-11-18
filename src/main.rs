use rand::Rng;
use std::io;

fn score(secret: u64, guess: u64) -> (usize, usize) {
    let to_digits = |number: u64| {
        [
            (number / 100) as u8,
            (number / 10 % 10) as u8,
            (number % 10) as u8,
        ]
    };

    let secret_digits = to_digits(secret);
    let guess_digits = to_digits(guess);
    let mut strikes = 0;
    let mut matched = [false; 3];
    let mut remaining_secret = [0_u8; 10];

    for index in 0..3 {
        if secret_digits[index] == guess_digits[index] {
            strikes += 1;
            matched[index] = true;
        } else {
            remaining_secret[secret_digits[index] as usize] += 1;
        }
    }

    let mut balls = 0;
    for index in 0..3 {
        if matched[index] {
            continue;
        }

        let digit = guess_digits[index] as usize;
        if remaining_secret[digit] > 0 {
            balls += 1;
            remaining_secret[digit] -= 1;
        }
    }

    (strikes, balls)
}

fn has_unique_digits(mut number: u64) -> bool {
    let mut seen = [false; 10];

    if number == 0 {
        return true;
    }

    while number > 0 {
        let digit = (number % 10) as usize;
        if seen[digit] {
            return false;
        }

        seen[digit] = true;
        number /= 10;
    }

    true
}

fn main() {
    println!("Put your 3-digit guess. Order matters.");

    let secret_number = loop {
        let number = rand::thread_rng().gen_range(100..=999);
        if has_unique_digits(number) {
            break number;
        }
    };

    let mut trial_count = 0;
    loop {
        println!("Please input your guess");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim();
        if guess.len() == 3 && guess.starts_with('0') {
            println!("The hundreds digit cannot be 0.");
            continue;
        }

        let guess: u64 = match guess.parse() {
            Ok(num) if (100..=999).contains(&num) && has_unique_digits(num) => num,
            Ok(num) if (100..=999).contains(&num) => {
                println!("Please enter three different digits.");
                continue;
            }
            Ok(_) => {
                println!("Please enter a 3-digit number.");
                continue;
            }
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };
        trial_count += 1;
        println!("Trial count: {trial_count}");
        let (strikes, balls) = score(secret_number, guess);
        match (strikes, balls) {
            (3, _) => {
                println!("3S0B, Great job!");
                break;
            }
            (strikes, balls) => println!("{strikes}S{balls}B"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{has_unique_digits, score};

    #[test]
    fn scores_strikes_and_balls() {
        assert_eq!(score(123, 321), (1, 2));
        assert_eq!(score(345, 123), (0, 1));
    }

    #[test]
    fn does_not_reuse_a_digit_for_multiple_matches() {
        assert_eq!(score(112, 121), (1, 2));
        assert_eq!(score(112, 111), (2, 0));
    }

    #[test]
    fn rejects_duplicate_digits() {
        assert!(has_unique_digits(123));
        assert!(has_unique_digits(102));
        assert!(has_unique_digits(1_023_456_789));
        assert!(!has_unique_digits(1_023_456_780));
        assert!(!has_unique_digits(10_123_456_789));
        assert!(!has_unique_digits(112));
        assert!(!has_unique_digits(121));
        assert!(!has_unique_digits(211));
    }
}
