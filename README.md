# Automation
Automation scripts to use


## How to Build

To build the Rust project, follow the steps below:

1. Make sure you have Rust installed. You can install Rust using rustup. Follow the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).
2. Navigate to the project directory where the `Cargo.toml` file is located.
3. To compile the project, run the following command:
   ```sh
   cargo build --release
   ```
   This will compile the project in release mode, generating an optimized binary.
4. To run the tests, execute:
   ```sh
   cargo test
   ```

Ensure all tests pass before considering the build successful.

## Config 
Configure the automation in your terminal alias
```sh
echo "alias automation='./target/release/automation'" >> ~/.zshrc
```
```sh
source ~/.zshrc
```

## How to use the project

```sh
automation uv .
```
or 

```sh
automation poetry .
```



