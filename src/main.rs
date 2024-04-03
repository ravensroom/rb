use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    temperature_converter();
}

fn temperature_converter() {
    loop {
        println!("Temperature in farenheit or celsius with F/C postfix:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();
        let converter = &input[input.len() - 1..];
        let converter = converter.to_uppercase();
        let temperature = &input[..input.len() - 1];
        let temperature: f64 = match temperature.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input: temperature must be a number.");
                return;
            }
        };
        if converter == "F" {
            let celsius = farenheit_to_celsius(temperature);
            println!("{}F is {}C", temperature, celsius);
        } else if converter == "C" {
            let farenheit = celsius_to_farenheit(temperature);
            println!("{}C is {}F", temperature, farenheit);
        } else {
            println!("Invalid input: temperature must have F or C postfix.");
        }
    }
}

fn farenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_farenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn play_guess_game() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
