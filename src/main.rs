use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the random number in the range [1,100]!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess (q for quit).");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess = guess.trim();

        let guess: u32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                if guess.eq("q") {
                    println!("Game stopped");
                    break;
                } else {
                    continue;
                }
            },
        };


        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break;
            },
        }
    }

}
