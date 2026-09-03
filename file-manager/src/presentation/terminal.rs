

//                        
//======== STDIN ========//
//                      
pub(super) mod input {
    use std::io::stdin;
    use crate::presentation::errors::InputError;

    pub fn read_u8() -> 
        Result<
            u8, 
            InputError
        > {
        let mut buf: String = String::new();
        stdin().read_line(&mut buf)?;
            
        Ok(buf.trim().parse::<u8>()?)
    }
    pub fn read_string() -> 
        Result<
            String, 
            InputError
        > {
        let mut buf: String = String::new();
        stdin().read_line(&mut buf)?;
    
        Ok(buf.trim().to_string())
    }
}


//                        
//======== STDOUT ========//
//                      

pub mod output {
    use std::{io::{ Write, stdout }};

    pub fn flush_stdout() -> std::io::Result<()> {
        stdout().flush()?;
        Ok(())
    }
    pub fn clear_screen() {
        print!("\x1b[2J\x1b[H");
    }
    pub fn input_indicator() {
        print!("> ");
    }
    pub fn prompt(msg: &str) {
        print!("{}", msg);
        let _ = flush_stdout();
    }
    pub fn display_result(result: &str) {
        print!("\x1b[1mResultado: \x1b[0m");
        println!("{}", result);
    }
}