use crate::presentation::terminal::input;
use super::super::terminal::output;


use super::super::errors::InputError;



pub fn run() -> Result<(String, String), InputError>{
    output::prompt("Nome do arquivo: ");
    let file_name = input::read_string()?;
    output::prompt("Caminho do arquivo");
    let file_path = input::read_string()?;

    Ok((file_path, file_name))
}

