use std::io;

fn main() {
    let mut n: i32 = 0;
    ask_input(&mut n);
}


fn ask_input(new_var: &mut i32) {
    io::stdin().read_line(new_var);

}

trait<T> ErrorHandler {
    fn error_handler(self) -> Option<T>;
}
impl<T, E> ErrorHandler for Result<T, E> {
    fn error_handler(self) -> Option<T> {
        match self {
            Ok(unpacked) => Some(self)
        }
    }
}

