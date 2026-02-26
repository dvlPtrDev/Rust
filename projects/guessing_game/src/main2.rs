
use std::io; // Use o módulo Input/output da biblioteca padrão Standard
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
    
    // Cria uma variável imutável com o valor retornado de read_input e passa como argumento a msg de erro
}
fn read_input() -> i32 {
    let mut guess = String::new(); // Declara uma string vazia
    io::stdin() // Retorna err ou Ok
    .read_line(&mut guess) // Lê o input e armazena se retornar Ok
    .expect("Error reading input"); // Se não Exibe a exception 
// Captura a entrada do usuário e salva o resultado dentro de guess como string
        let guess: i32 = match guess.trim().parse() { // Sobreescreve o valor antigo de guess (Str)
        // trim() Apaga espaços extras no início e fim
        // parse() converte para o tipo em let guess: __
            Ok(num) => { 
                num;
                break;
            } Err(_) => {
            println!("Non-numeric value, error!");
            continue,
            }
        }
    return guess;
}