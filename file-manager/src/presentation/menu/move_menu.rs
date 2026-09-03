use std::println;

use crate::presentation::terminal::input;
use super::super::terminal::output;

use super::super::errors::InputError;



pub fn run() -> Result<(String, String), InputError>{
    disclaimer();

    output::prompt("Origem do arquivo: ");
    let origin = input::read_string()?;
    output::prompt("Mover para: ");
    let dest = input::read_string()?;

    let result_str = format!("Arquivo movido de {} para {}", origin, dest);
    output::display_result(&result_str);

    Ok((origin, dest))
}

fn disclaimer() {
    println!("Para renomear: ");
    println!("/caminho/arquivo /caminho/arquivo_renomeado.txt");
    println!("Para mover");
    println!("/caminho1/arquivo /caminho2/arquivo");
}
