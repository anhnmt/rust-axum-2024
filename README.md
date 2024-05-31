# rust-axum-2024

## Getting Started
### Prerequisites

If you to want the feature to run in watch mode, install `cargo-watch`
* cargo-watch
  ```sh
  cargo install cargo-watch
  ```

### Installation
1. Clone the repo
   ```sh
   git clone https://github.com/anhnmt/rust-axum-2024.git
   ```
2. Build the project to install the dependencies
   ```sh
   cargo build
   ```
4. Run with
   ```sh
   cargo run
   ```
   Or, for more dynamic development, run in watch mode
   ```sh
   cargo watch -q -c -w src/ -x run
   ```
   - `-q` quiet mode
   - `-c` clear the console on reload
   - `-w` watch all change to files under `src/`
   - `-x` executes `run` command