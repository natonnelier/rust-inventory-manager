# Rust::Inventory Manager

Desktop App for shops inventory management, developed using [Rust Nightly](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) and [Rocket](https://github.com/SergioBenitez/Rocket). Compatible with macOS, Linix and Windows.

## Installation

You will need sqlite for DB and [rustup](https://rustup.rs/) to install a nightly version of Rust.

### macOS
  * `brew install sqlite`
  * `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
  * `rustup default nightly`

### Debian, Ubuntu
  * `apt-get install libsqlite3-dev`
  * `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
  * `rustup default nightly`

### Arch
  * `pacman -S sqlite`
  * `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
  * `rustup default nightly`

## Run

### All platforms

  * `cargo run`