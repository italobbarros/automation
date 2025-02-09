use std::fs;
use std::path::PathBuf;
use crate::python::commands as python;
use crate::uv::commands as uv;
use crate::poetry::commands as poetry;


pub fn verify(is_uv: bool, is_poetry: bool,python_version: &str, dir_path: &PathBuf) {
let python_version_file = dir_path.join(".python-version");
    if python_version_file.exists() {
        if let Ok(version) = fs::read_to_string(python_version_file) {
            let version = version.trim();
            if is_uv{
                if uv::is_valid_python_version(&python_version, version) {
                    println!("A versão do Python é compatível: {}", version);
                } else {
                    println!("A versão do Python não é compatível: {}", version);
                    python::install_or_set_python_version(is_uv,is_poetry, &python_version, &dir_path);
                }
            }
            else if is_poetry{
                if poetry::is_valid_python_version(&python_version, version) {
                    println!("A versão do Python é compatível: {}", version);
                } else {
                    println!("A versão do Python não é compatível: {}", version);
                    python::install_or_set_python_version(is_uv,is_poetry, &python_version, &dir_path);
                }
            }
        } else {
            println!("Erro ao ler o arquivo .python-version");
        }
    } else {
        println!("Arquivo .python-version não encontrado. Baixando versão {}...", python_version);
        python::install_or_set_python_version(is_uv,is_poetry, &python_version, &dir_path);
    }
}