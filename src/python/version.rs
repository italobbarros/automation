use std::fs;
use std::path::PathBuf;
use crate::uv::commands as uv;
use crate::poetry::commands as poetry;
use crate::cli as cli;
pub fn get_python_version(package: &cli::Package, dir_path: &PathBuf) -> String {
    let mut python_version = String::from("3.x");
    if let Ok(contents) = fs::read_to_string(dir_path.join("pyproject.toml")) {
        match package {
            cli::Package::UV => {
                python_version = uv::get_python_version(&contents);
            },
            cli::Package::POETRY => {
                python_version = poetry::get_python_version(&contents);
            },
        }
    }
    python_version
}
