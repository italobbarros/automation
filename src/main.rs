use automation::python::pyenv as pyenv;
use automation::python::venv as venv;
use automation::python::version as version;
use automation::cli as cli; 

fn main() {
    let config: Option<cli::Config>   = cli::parse_args();   
    if let Some(config) = config {
        println!("Checking directory: {}", config.dir_path.display());
        // Check for lock files in the specified directory
        let python_version = version::get_python_version(config.is_uv, config.is_poetry, &config.dir_path);
        println!("Python version: {}", python_version);

        // Verify pyenv version
        pyenv::verify(config.is_uv,config.is_poetry, &python_version, &config.dir_path);

        // Check for virtual environment directories
        venv::verify(config.is_uv,config.is_poetry, &config.dir_path);
    }
}
