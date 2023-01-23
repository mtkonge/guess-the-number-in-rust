use rand::Rng;

fn random_number_1_to_100() -> u8 {
    rand::thread_rng().gen_range(0..100)
}

fn get_input() -> u8{
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let input = line.trim().parse().unwrap();
    input
}

fn main() {
    let number_to_guess = random_number_1_to_100();
    let mut number_guessed = false;
    while !number_guessed {
        println!("Guess the number from 0-100");
        let guessed_number = get_input();
        if guessed_number > number_to_guess {
            println!("The number is lower");
        } else if guessed_number < number_to_guess {
            println!("The number is higher");
        } else {
            number_guessed = true;
            println!("You guessed the number!");
        }
    }
}
