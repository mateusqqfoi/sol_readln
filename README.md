# SOL_READLN

A simple alternative to Rust's standard library's method of **reading input** from the terminal + a print function that automatically flushes the stdout like in other languages. 

*(Made for learning and people coming from higher level languages.)*

## Usage
```rust
use sol_readln::*;

fn main() {
    /// Prompts the user (terminal) for input, returns it as a String.
    /// Panics if it could not read it.
    let input = readln();

    /// Prompts the user (terminal) for input and returns a Result.
    /// Ok(input) where input is a String or
    /// Err(error) where error is a std::io::error:Error.
    let input = read_line();

    /// Prints and flushes the stdout, will panic if it isn't possible.
    /// Simple higher-level alternative, works as expected in other
    /// higher-level languages.
    print("Hello, sol!");
}
```