use std::path::PathBuf;
use crate::cli as cli;

pub fn verify(package: &cli::Package, dir_path: &PathBuf) {
    let has_dot_venv = dir_path.join(".venv").exists();
    let has_env = dir_path.join("env").exists();
    let has_venv = dir_path.join("venv").exists();

    if !has_dot_venv && !has_env && !has_venv {
        set_venv_folder(dir_path);
        match package {
            &cli::Package::UV => {
                println!("uv sync");
            }
            &cli::Package::POETRY => {
                println!("poetry install");
            }
        }
    }
}
fn set_venv_folder(dir_path: &PathBuf) {
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg(format!("cd {} && python3 -m venv .venv", dir_path.display()))
        .status()
        .expect("Failed to create virtual environment");

    if status.success() {
        println!("Successfully created virtual environment");
    } else {
        eprintln!("Failed to create virtual environment");
    }
}

// fn activate_venv(dir_path: &PathBuf) {
//     let has_dot_venv = dir_path.join(".venv").exists();
//     let has_env = dir_path.join("env").exists();
//     let has_venv = dir_path.join("venv").exists();

// }

// fn deactivate_venv(dir_path: &PathBuf) {
//     let has_dot_venv = dir_path.join(".venv").exists();
//     let has_env = dir_path.join("env").exists();
//     let has_venv = dir_path.join("venv").exists();
// }

