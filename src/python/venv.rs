use std::path::PathBuf;
use crate::cli as cli;

pub fn verify(package: &cli::PackageMode, dir_path: &PathBuf) {
    let has_dot_venv = dir_path.join(".venv").exists();
    let has_env = dir_path.join("env").exists();
    let has_venv = dir_path.join("venv").exists();

    if !has_dot_venv && !has_env && !has_venv {
        set_venv_folder(dir_path);
        match package {
            &cli::PackageMode::UV => {
                println!("uv sync");
                let status = std::process::Command::new("uv")
                    .arg("sync")
                    .status()
                    .expect("Failed to run uv sync");
                if !status.success() {
                    println!("uv sync failed");
                    return;
                }
                println!("uv sync completed successfully");
                create_envrc(dir_path, ".venv");
                set_python_interpreter(dir_path, ".venv");
            }
            &cli::PackageMode::POETRY => {
                println!("poetry install");
                let status = std::process::Command::new("poetry")
                    .arg("install")
                    .arg("--no-root")
                    .status()
                    .expect("Failed to run poetry install");
                if !status.success() {
                    println!("poetry install failed");
                    return;
                }
                println!("poetry install completed successfully");
                create_envrc(dir_path, ".venv");
                set_python_interpreter(dir_path, ".venv");
            }
        }
    }
}
fn set_venv_folder(dir_path: &PathBuf) {
    let prompt_name = dir_path
        .components()
        .next_back()
        .and_then(|comp| comp.as_os_str().to_str())
        .unwrap_or(".venv");

    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg(format!("cd {} && python3 -m venv .venv --prompt {}", dir_path.display(), prompt_name))
        .status()
        .expect("Failed to create virtual environment");

    if status.success() {
        println!("Successfully created virtual environment");
    } else {
        eprintln!("Failed to create virtual environment");
    }
}

fn create_envrc(dir_path: &PathBuf, venv_name: &str) {
    let envrc_path = dir_path.join(".envrc");
    let envrc_content = format!("source {}/{}/bin/activate", dir_path.display(), venv_name);
    
    if envrc_path.exists() {
        println!("{} already exists", envrc_path.display());
        return;
    }

    match std::fs::write(&envrc_path, envrc_content) {
        Ok(_) => println!("Successfully created .envrc"),
        Err(e) => eprintln!("Failed to create .envrc: {}", e),
    }
    let status = std::process::Command::new("direnv")
        .arg("allow")
        .status()
        .expect("Failed to run direnv allow");

    if status.success() {
        println!("Successfully ran direnv allow");
    } else {
        eprintln!("Failed to run direnv allow");
    }
}

fn set_python_interpreter(dir_path: &PathBuf, venv_name: &str) {
    let venv_python_path = dir_path.join(venv_name).join("bin").join("python");
    let vscode_settings_path = dir_path.join(".vscode").join("settings.json");

    let settings_content = format!(
        r#"{{
    "python.pythonPath": "{}"
}}"#,
        venv_python_path.display()
    );

    std::fs::create_dir_all(dir_path.join(".vscode")).expect("Failed to create .vscode directory");

    match std::fs::write(&vscode_settings_path, settings_content) {
        Ok(_) => println!("Successfully set Python interpreter in VSCode to {}", venv_python_path.display()),
        Err(e) => eprintln!("Failed to set Python interpreter in VSCode: {}", e),
    }
}


