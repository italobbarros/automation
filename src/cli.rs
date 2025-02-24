use std::path::PathBuf;
use std::fs;
use std::env;

pub enum PackageMode {
    UV,
    POETRY,
}

pub struct Package {
    pub dir_path: PathBuf,
    pub package_mode: PackageMode,
    pub python_version: String,
}

pub struct Safari {
    pub cmd: Option<String>,
    pub tags: Option<Vec<String>>,
}

pub struct AutomationParameters {
    pub package: Option<Package>,
    pub safari: Option<Safari>,
}

pub fn parse_args() -> Option<AutomationParameters> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        if args.len() == 2 && (args[1] == "-h" || args[1] == "--help") {
            print_help();
            return None;
        }
        println!("Usage: automation --help");
        return None;
    }

    if package_parse(&args[1]).is_some() { //Verify automation is uv or poetry
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


        Some(AutomationParameters{
            package: Some(Package {
                dir_path,
                package_mode: package_parse(&args[1]).unwrap(),
                python_version: String::from("3.x"), // valor padrão
            }),
            safari: None,
        })
    } else { // Verify if automation is open safari
        let mut cmd = None;
        let mut tags = None;

        for i in 1..args.len() {
            if args[i] == "--tags" || args[i] == "-t" {
                if i + 1 < args.len() {
                    tags = Some(args[i + 1].split(',').map(|s| s.to_string()).collect());
                } else {
                    println!("Error: --tags or -t specified but no tags provided");
                    return None;
                }
            } else {
                cmd = Some(args[i].clone());
            }
        }
        Some(AutomationParameters{
            package: None,
            safari: Some(Safari {
                cmd: cmd,
                tags: tags,
            }),
        })
    }
}


pub fn package_parse(package: &str) -> Option<PackageMode> {
    match package {
        "uv" => Some(PackageMode::UV),
        "poetry" => Some(PackageMode::POETRY),
        _ => None,
    }
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

