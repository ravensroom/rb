use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    print_12_days_chrismas();
}

/**
*  On the first day of Christmas my true love sent to me
   A partridge in a pear tree

   On the second day of Christmas my true love sent to me
   Two turtle doves
   And a partridge in a pear tree

   On the third day of Christmas my true love sent to me
   Three French hens
   Two turtle doves
   And a partridge in a pear tree
*/
fn print_12_days_chrismas() {
    let sections = [0, 1, 2];
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
    ];
    for section in sections {
        let section_name = match section {
            0 => "first",
            1 => "second",
            2 => "third",
            _ => return,
        };
        println!(
            "On the {} day of Christmas my true love sent to me",
            section_name
        );
        for i in (0..=section).rev() {
            if (i == 0) && (section != 0) {
                println!("And {}", gifts[i].to_lowercase());
            } else {
                println!("{}", gifts[i]);
            }
        }
        println!();
    }
}

fn print_nth_fibonacci() {
    loop {
        println!("Enter the nth fibonacci number you want to find:");
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Failed to read line");
        let n: u64 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input: n must be a number.");
                continue;
            }
        };
        let fibonacci = nth_fibonacci(n);
        println!("The {}th fibonacci number is {}", n, fibonacci);
    }
}

fn nth_fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0;
            let mut b = 1;
            for _ in 2..n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
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
                continue;
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
