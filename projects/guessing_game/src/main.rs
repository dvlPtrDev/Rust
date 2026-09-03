
use std::io; 
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("\n\n\n\t\t  ==================== Guessing game ==================== ");
    println!("Type a number: ");
    check_result()
}

fn check_result() {
    let secret_number = rand::rng().random_range(1..=100);
    loop {
        let guess = read_input(); 
        if guess > 100 || guess < 1 {
            println!("Enter a number between 1 - 100")
        }
        match guess.cmp(&secret_number) {
    
            Ordering::Less => println!("Try a greater number"),
            Ordering::Greater => println!("Try a smaller number"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
fn read_input() -> u32 {
    loop {
        let mut guess = String::new(); 
        io::stdin() 
        .read_line(&mut guess)
        .expect("Error reading input");
            match guess.trim().parse::<u32>() { 
                Ok(typed) => break typed,
                Err(_) => {
                    println!("Non-typed numeric value, error!");
                    continue;
               }
            };
    }
}   