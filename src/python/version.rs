use std::fs;
use std::path::PathBuf;
use crate::uv::commands as uv;
use crate::poetry::commands as poetry;

pub fn get_python_version(is_uv: bool, is_poetry: bool, dir_path: &PathBuf) -> String {
    let mut python_version = String::from("3.x");
    if let Ok(contents) = fs::read_to_string(dir_path.join("pyproject.toml")) {
        if is_uv {
            python_version = uv::get_python_version(&contents);
        } else if is_poetry {
            python_version = poetry::get_python_version(&contents);
        }
    }
    python_version
}
