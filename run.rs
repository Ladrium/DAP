// Commandline to Compile either Rust or C++ in P..
use std::{env, io, process};

fn compile(file: String, file_type: &str) -> io::Result<()> {
    let dir = env::current_dir()?;

    let command = match file_type {
        "rs" => "rustc",
        "cpp" => "g++",
        _ => "rustc",
    };

    let formatted_file = file.replace('.', "-");
    let input = format!(
        "{}/{}/{}.{}",
        dir.display(),
        file,
        formatted_file,
        file_type
    );
    let output = format!("{}/{}/{}.exe", dir.display(), file, formatted_file);

    let cmd = process::Command::new(command)
        .arg(input)
        .arg("-o")
        .arg(&output)
        .output()?;

    if cmd.status.success() {
        println!("Compiled!");
        execute(output);
        Ok(())
    } else {
        eprintln!("Compilation failed: {:?}", cmd);
        Err(io::Error::new(io::ErrorKind::Other, "Compilation failed"))
    }
}

fn execute(file_path: String) {
    process::Command::new(file_path)
        .status()
        .expect("Failed to execute");

    println!("Finished Program!");
}

fn main() -> io::Result<()> {
    let mut args = env::args();
    args.next();
    let len = args.len();
    let run_type: String;

    if len == 0 {
        println!("Aufgabe nicht gegeben: <run> -r|-c <Aufgabe(P5.1)>");
        return Ok(());
    } else if len == 1 {
        run_type = String::from("-c");
    } else {
        run_type = args.next().unwrap();
    }

    let file = args.next().unwrap();

    match run_type.to_lowercase().as_str() {
        "-c" => {
            compile(file, "cpp")?;
        }
        "-r" => {
            compile(file, "rs")?;
        }
        _ => {
            println!("Aufgabe nicht gegeben: <run> -r|-c <Aufgabe(P5.1)>");
        }
    };

    Ok(())
}
