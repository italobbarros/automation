use std::path::PathBuf;
use std::fs;
use std::env;

pub struct Config {
    pub dir_path: PathBuf,
    pub is_uv: bool,
    pub is_poetry: bool,
    pub python_version: String,
}

pub fn parse_args() -> Option<Config> {
    let args: Vec<String> = env::args().collect();
    let dir_path: PathBuf = if args.len() > 1 {
        match fs::canonicalize(&args[1]) {
            Ok(path) => path,
            Err(e) => {
                println!("Error processing path: {}", e);
                return None;
            }
        }
    } else {
        println!("Usage: cargo run <directory_path>");
        return None;
    };

    // Verificações básicas
    if !dir_path.join("pyproject.toml").exists() {
        println!("No pyproject.toml found in {}", dir_path.display());
        return None;
    }

    let is_uv = dir_path.join("uv.lock").exists();
    let is_poetry = dir_path.join("poetry.lock").exists();

    if !is_uv && !is_poetry {
        println!("No uv.lock or poetry.lock found in {}", dir_path.display());
        return None;
    }

    Some(Config {
        dir_path,
        is_uv,
        is_poetry,
        python_version: String::from("3.x"), // valor padrão
    })
}
