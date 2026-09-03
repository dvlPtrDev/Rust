fn main() {
    let mut counter = 0;
    'counting: loop {
        let mut remaining = 10;
        println!("counter = {counter}");
        if counter >= 10 {
            println!("\n\n\nBye!\n\n\n");
            break;
        }
        println!("Again!");
        
        loop {
            counter += 1;
            println!("Remaining = {remaining}");
            if remaining > 5 {
                remaining -= 1;
                continue;
            } else {        
                println!("counter = {counter}");
                break 'counting;
            }
        }
    };
    println!("\n{counter}\n");
}
