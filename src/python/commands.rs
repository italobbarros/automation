use std::path::PathBuf;
use std::process::Command;
use std::str;
use crate::uv::commands as uv;
use crate::poetry::commands as poetry;


pub fn list_pyenv_versions() -> Vec<String> {
    let output = Command::new("pyenv")
        .arg("install")
        .arg("--list")
        .output()
        .expect("Failed to execute pyenv command");

    let output_str = str::from_utf8(&output.stdout).expect("Failed to convert output to string");
    output_str.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim().to_string())
        .collect()
}

pub fn find_compatible_version(is_uv:bool,is_poetry:bool, requirement: &str) -> Option<String> {
    let versions = list_pyenv_versions();
    let mut compatible_versions: Vec<String> = Vec::new();
    if is_uv{
        compatible_versions = versions.into_iter()
        .filter(|version| uv::is_valid_python_version(requirement, version))
        .collect();
    }
    else if is_poetry{
        compatible_versions = versions.into_iter()
        .filter(|version| poetry::is_valid_python_version(requirement, version))
        .collect();
    }
    compatible_versions.sort_by(|a, b| {
        let a_parts: Vec<u32> = a.split('.').filter_map(|s| s.parse().ok()).collect();
        let b_parts: Vec<u32> = b.split('.').filter_map(|s| s.parse().ok()).collect();
        b_parts.cmp(&a_parts)
    });

    compatible_versions.first().cloned()
}

pub fn is_python_version_installed(version: &str) -> bool {
    let output = Command::new("pyenv")
        .arg("versions")
        .output()
        .expect("Failed to execute pyenv versions command");

    let output_str = str::from_utf8(&output.stdout).expect("Failed to convert output to string");
    output_str.lines().any(|line| line.trim().starts_with(version))
}

pub fn set_local_python_version(version: &str, path: &PathBuf) {
    let status = Command::new("sh")
        .arg("-c")
        .arg(format!("cd {} && pyenv local {}", path.display(), version))
        .status()
        .expect("Failed to execute pyenv local command");

    if status.success() {
        println!("Successfully set local Python version to {}", version);
    } else {
        eprintln!("Failed to set local Python version to {}", version);
    }
}

pub fn install_or_set_python_version(is_uv: bool,is_poetry:bool, requirement: &str, path: &PathBuf) {
    if let Some(version) = find_compatible_version(is_uv,is_poetry, requirement) {
        if is_python_version_installed(&version) {
            set_local_python_version(&version, path);
        } else {
            let status = Command::new("pyenv")
                .arg("install") 
                .arg(&version)
                .status()
                .expect("Failed to execute pyenv install command");

            if status.success() {
                println!("Successfully installed Python version {}", version);
                set_local_python_version(&version, path);
            } else {
                eprintln!("Failed to install Python version {}", version);
            }
        }
    } else {
        eprintln!("No compatible Python version found for requirement {}", requirement);
    }
}
