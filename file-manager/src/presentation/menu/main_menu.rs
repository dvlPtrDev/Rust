use super::super::terminal::input;
use super::super::terminal::output;

use super::super::errors::InputError;

pub fn run() -> 
    Result<
        u8, 
        InputError
    > {
    output::clear_screen();
    self::display();
    output::input_indicator();
    output::flush_stdout()?;
    input::read_u8()
}

fn display()  {
    println!("Gerenciador de arquivos");
    println!("[1] Criar arquivo");
    println!("[2] Deletar arquivo");
    println!("[3] Mover arquivo");
}

