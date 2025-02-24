use automation::python::pyenv as pyenv;
use automation::python::venv as venv;
use automation::python::version as version;
use automation::cli as cli; 
use automation::safari::safari as safari;

fn main() {
    let automation: Option<cli::AutomationParameters> = cli::parse_args();
    if let Some(automation) = automation {
        if automation.package.is_some() {
            let package = automation.package.unwrap();
            println!("Checking directory: {}", package.dir_path.display());
            // Check for lock files in the specified directory
            let python_version = version::get_python_version(&package.package_mode, &package.dir_path);
            println!("Python version: {}", python_version); 
            // Verify pyenv version
            pyenv::verify(&package.package_mode, &python_version, &package.dir_path);
            // Check for virtual environment directories
            venv::verify(&package.package_mode, &package.dir_path);
        }
        if automation.safari.is_some() {
            let safari = automation.safari.unwrap();
            if safari.cmd.is_some() {
                safari::open_safari_with_cmd(&safari.cmd.unwrap());
            }
            if safari.tags.is_some() {
                safari::open_safari_with_tags(&safari.tags.unwrap());
            }
        }
    }
}
