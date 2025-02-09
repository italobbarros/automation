use std::path::PathBuf;


pub fn verify(is_uv: bool, is_poetry: bool, dir_path: &PathBuf) {
    let has_dot_venv = dir_path.join(".venv").exists();
    let has_env = dir_path.join("env").exists();
    let has_venv = dir_path.join("venv").exists();

    if !has_dot_venv && !has_env && !has_venv {
        println!("python -m venv .venv");
        if is_uv {
            println!("uv sync");
        } else if is_poetry {
            println!("poetry install");
        }
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

