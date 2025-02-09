use std::path::PathBuf;
use std::fs;
use std::env;

pub enum Package {
    UV,
    POETRY,
}

pub struct Config {
    pub dir_path: PathBuf,
    pub package: Package,
    pub python_version: String,
}

pub fn parse_args() -> Option<Config> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        if args.len() == 2 && (args[1] == "-h" || args[1] == "--help") {
            print_help();
            return None;
        }
        println!("Usage: automation <package> <directory_path>");
        println!("or");
        println!("Usage: automation --help");
        return None;
    }

    let package = match args[1].as_str() {
        "uv" => Package::UV,
        "poetry" => Package::POETRY,
        _ => {
            println!("Invalid package type. Use 'uv' or 'poetry'.");
            return None;
        }
    };

    let dir_path: PathBuf = match fs::canonicalize(&args[2]) {
        Ok(path) => path,
        Err(e) => {
            println!("Error processing path: {}", e);
            return None;
        }
    };

    // Verificações básicas
    if !dir_path.join("pyproject.toml").exists() {
        println!("No pyproject.toml found in {}", dir_path.display());
        return None;
    }


    Some(Config {
        dir_path,
        package,
        python_version: String::from("3.x"), // valor padrão
    })
}



pub fn print_help() {
    println!("Usage: automation <package> <directory_path>");
    println!();
    println!("Commands:");
    println!("  uv <directory_path>       Set up the UV package in the specified directory");
    println!("  poetry <directory_path>   Set up the Poetry package in the specified directory");
    println!();
    println!("Options:");
    println!("  -h, --help                Print this help message and exit");
}

