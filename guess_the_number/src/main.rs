use rand::Rng;
use std::cmp::Ordering;
use std::io::Write;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut guess = String::new();
        print!("\nGuess the number?\n> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut guess).unwrap();
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small."),
            Ordering::Greater => println!("{}", "Too big."),
            Ordering::Equal => {
                println!("{}", "You win!\n");
                break;
            }
        }
    }
}
