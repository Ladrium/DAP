// Commandline to Compile either Rust or C++ in P..
use std::{env, io};

fn compile_rust() -> io::Result<()> {
    // TODO! Compile Rust
    Ok(())
}

fn compile_cpp() -> io::Result<()> {
    // TODO! Compile C++
    Ok(())
}

fn main() -> io::Result<()> {
    let run_type = env::args().nth(1)

    match run_type.as_str() {
        "-c" => {
            compile_cpp()?;
                }
        "-r" | _ => {
            compile_rust()?;
        }
    };

    Ok(())
}
