use std::io;
use  std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Fail to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_err) => {
                println!("Invalid number format");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low"),
            Ordering::Greater => println!("Too high") ,
            Ordering::Equal => {
                println!("You got it right!");
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::thread_rng;

    #[test]
    fn test_generate_secret_number() {
        let secret_number = thread_rng().gen_range(1..=100);
        assert!(secret_number >= 1 && secret_number <= 100);
    }

    #[test]
    fn test_parse_valid_guess() {
        let guess = "42";
        let parsed_guess: u32 = guess.trim().parse().unwrap();
        assert_eq!(parsed_guess, 42);
    }

    #[test]
    fn test_parse_invalid_guess() {
        let guess = "abc";
        let result: Result<u32, _> = guess.trim().parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_compare_guess_to_secret_number() {
        let secret_number = 50;
        let guess = 40;
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => assert!(true),
            _ => assert!(false),
        }
    }
}
