# Infromation

This is my version of class-1 code; it includes my code, my docs and my notes. You can checkout class-1's original code [here](https://github.com/CocDap/Rust-Bootcamp-2023/tree/class-1-code-content). With class-1 slide [here](https://docs.google.com/presentation/d/1WSBMMdnsJxGguUS13v9U99FH_Z2pFTMC/edit#slide=id.p1) and class-1 video lesson [here](https://youtu.be/Udsqw_zjKLY).

The content of class-1 is the basics about rust.

You **must** checkout my class-1 docs for some notes that I have created while learning.

You should use `cargo clean` to clean up unneeded files.

# Cargo Packages

You need to install cargo make via `cargo install cargo-make`.

You need to install cargo watch via `cargo install cargo-watch`. This is an auto-compiler, similar to nodemon on NodeJS. You can run it with this command `cargo watch -x check -x test -x run` or this command `cargo watch -x check -x run`.

Use this command `cargo install --list` to check if you have installed the cargo packages correcttly.

# Exensions

You need to install rust-analyzer for Vscode's WSL version only.

You need to isntall Error Lens for Vscode's local/global version.

# Step by Step

Create a new rust project: `cargo new <project-name>`.

Locate to directory: `cd <project-name>`.

Check the project for any errors: `cargo check`.

Run the project, not optimized: `cargo run`.

**Note:** Since you have downloaded `cargo-watch`, you can run the above 2 commands via 1 line of command: `cargo watch -x check -x run` or `cargo watch -x check -x test -x run` (We recommend the first command, for now).

# Warning

There are no warnings yet. More updates will come.
