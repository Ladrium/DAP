// Commandline to Compile either Rust or C++ in P..
use std::{io, env};

fn main() -> io::Result<()> {
    let run_type = env::args().nth(1).unwrap();

    match run_type {
        "r" | _: {

    }, 
        "c" | 
    }

    Ok(())
}
