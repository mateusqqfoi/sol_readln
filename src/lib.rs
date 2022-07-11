use std::io::{self, Error, Write};

/// Prompts the user (terminal) for input and returns it as a String.
/// Panics if it could not read it.
pub fn readln() -> String {
    let mut s: String = String::new();
    
    io::stdin().read_line(&mut s)
               .expect("Could not read input.");
    
    s.trim().to_string()
}

/// Prompts the user (terminal) for input and returns a Result.
/// Ok(input) where input is a String or
/// Err(error) where error is a std::io::error:Error.
pub fn read_line() -> Result<String, Error> {
    let mut s: String = String::new();
    
    let r = io::stdin().read_line(&mut s);   
    
    match r {
        Ok(_)  => Ok(s.trim().to_string()),
        Err(e) => Err(e)
    }
}

/// Prints and flushes the stdout, will panic if it isn't possible.
/// Simple higher-level alternative, works as expected in other
/// higher-level languages.
pub fn print_flush(s: &str) {
    print!("{}", s);
    io::stdout().flush()
                .expect("Could not flush the stdout.");
}
