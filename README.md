# Information

The title of class-2's lesson is **Owning and Borrowing**.

This is my version of class-2 code, I called it my-class-2. It includes my source code, my docs and my notes. 

You can checkout class-2's original code [here](https://github.com/CocDap/Rust-Bootcamp-2023/tree/class-2-code-content). 

You can checkout class-2's resources slide [here](https://docs.google.com/presentation/d/1AzU_sCEEZw2fxL0LK3oNIURkku4Hzn-2/edit#slide=id.p1).
 
You can checkout class-2's video lesson [here](https://youtu.be/vNRJ2t7pzgs).

There are some additional notes that I can't integrate within the lesson, so I have noted them down onto `my-class-2-docs.txt`. Please check it out for more information.

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
