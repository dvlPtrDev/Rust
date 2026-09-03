mod infrastructure;
mod application;
mod presentation;
mod domain;

use std::format;

use crate::domain::errors::Errors;

use crate::infrastructure::os_adapter::{self, terminate};
use crate::presentation::menu::{
    create_menu, delete_menu, main_menu, move_menu
};

use crate::application::{create_file, delete_file, move_file};
use crate::presentation::terminal::output;

fn main() {
    if let Err(error) = run() {
        eprintln!("Erro: {error}");
        terminate(0);
    }
}

fn run() -> Result<(), Errors> {
    let option: u8;
    // loop {
        option = main_menu::run()?;
        
        match option {
            1 => {
                let (path, name) = create_menu::run()?;

                let file = create_file::create_file(&path, &name)?;

                let result_str = format!("Arquivo {} criado", file);
                output::display_result(&result_str);
            },
            2 => {
                let file_path = delete_menu::run()?;
                delete_file::delete_file(&file_path)?;

                let result_str = format!("Arquivo {} deletado", file_path);
                output::display_result(&result_str);
            }
            3 => {
                let (src, dest) = move_menu::run()?;

                move_file::move_file(&src, &dest)?;

                let result_str = format!("Arquivo {} movido para {}", src, dest);
                output::display_result(&result_str);
            },
            4 => os_adapter::terminate(0),
            _ => os_adapter::terminate(0),
        }
    // }
    Ok(())
}
