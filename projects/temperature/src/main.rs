use std::io::stdin;
use std::io::{ Write, stdout };


fn readln(mut var: String) -> String {
    loop {
        print!("> ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut var)
                .expect("Error reading line, try again!!!");
        return var;
    }
}

fn main() {
    println!("------- Conversor de temperatura -------");
    conversion_type()
}

fn conversion_type() {
    println!("Qual tipo de conversor você deseja usar? ");
    println!("[1] Celsius - Fahrenheit");
    println!("[2] Fahrenheit - Celsius");
    loop {
        let mut mode = String::new();
        mode = readln(mode);

        match mode.trim().parse::<u8>() {
            Ok(num) if num == 1 => {
                get_celsius();
                break;
            }
            Ok(num) if num == 2 => {
                get_fahrenheit();
                break;
            }
            Ok(_) => println!("Apenas 1 e 2 são valores permitidos"),
            Err(e) => println!("Valor inválido, {e}"),
        }       
    }
}
fn get_celsius() {
    println!("Valor em graus Celsius: ");
    loop {
        let celsius = String::new();
        let celsius = readln(celsius);
        match celsius.trim().parse::<i16>() {
            Ok(temp) => to_fahrenheit(temp),
            Err(e) => {
                println!("Erro!!! {e}");
                break;
            },    
        }
    }
}

fn to_celsius(fahrenheit: i16) {
    let celsius: i16 = (fahrenheit - 32) * 5/9;
    print_results(celsius, fahrenheit);
}

fn get_fahrenheit() {
    println!("Temperaturas em graus fahrenheit: ");
    loop {
        let fahrenheit = String::new();
        let fahrenheit = readln(fahrenheit);
        match fahrenheit.trim().parse::<i16>() {
            Ok(temp) => {
                to_celsius(temp);
                break;
            },
            Err(err) => println!("Erro: {err}"),
        }
    }
}

fn to_fahrenheit(celsius: i16) {
    let fahrenheit: i16 = (celsius * 9/5) + 32;
    print_results(celsius, fahrenheit);
}

fn print_results(celsius: i16, fahrenheit: i16) {
    println!("Temperatura em Graus Celsius: {celsius}°C");
    println!("Temperatura em Graus Fahrenheit: {fahrenheit}°F");
    return;
}