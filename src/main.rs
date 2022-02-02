use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess a number");

    let random_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("The random number is {}", random_number);
        println!("Enter in input");
    
        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess).expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&random_number) {
            Ordering::Less => println!("To small"),
            Ordering::Equal => {
                println!("You guessed it right!");
                break;
            },
            Ordering::Greater => println!("Too big"),
        }
        println!("The input is {}", guess);
    }
    
}
