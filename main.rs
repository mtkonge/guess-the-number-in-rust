use rand::prelude::*;


fn main() {
    let mut rng = rand::thread_rng();
    let random_float_0_to_1: f64 = rng.gen();
    let random_float_0_to_100: f64 = random_float_0_to_1 * 100.0;
    let number_to_guess: f64 = random_float_0_to_100.floor();
    let mut number_guessed = false;
    while !number_guessed {
        let mut line = String::new();
        println!("Guess the number from 0-100");
        std::io::stdin().read_line(&mut line).unwrap();
        let number: f64 = line.trim().parse().unwrap();
        if number > number_to_guess {
            println!("The number is lower");
        }
        else if number < number_to_guess {
            println!("The number is higher");
        }
        else {
            number_guessed = true;
            println!("You guessed the number!");
        }
    }
}