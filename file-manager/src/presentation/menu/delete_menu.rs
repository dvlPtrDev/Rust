use crate::presentation::terminal::input;
use super::super::terminal::output;

use super::super::errors::InputError;



pub fn run() -> Result<String, InputError>{
    output::prompt("Caminho e nome do arquivo");
    let file = input::read_string()?;

    Ok(file)
}

