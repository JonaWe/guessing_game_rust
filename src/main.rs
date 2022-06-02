use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn get_upper_bound() -> u32 {
    println!("Choose between 3 difficulties:");

    println!("\te = easy [1,10]");
    println!("\tm = medium [1,100]");
    println!("\th = hard [1,1000]");

    loop {
        let mut difficulty = String::new();

        io::stdin()
            .read_line(&mut difficulty)
            .expect("failed to read line");

        let upper_bound: u32 = match difficulty.trim() {
            "e" => 10,
            "m" => 100,
            "h" => 1000,
            _ => {
                println!("invalid input (choose e, m or h)");
                continue;
            },
        };

        return upper_bound;
    }
}

fn main() {
    let upper_bound = get_upper_bound();

    println!("Guess the random number in the range [1,{}]!", upper_bound);

    let secret_number = rand::thread_rng().gen_range(1..=upper_bound);

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
