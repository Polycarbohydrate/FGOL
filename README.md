# Ferris's Game of Life
<img width="1280" height="720" alt="Game of Life (2)" src="https://github.com/user-attachments/assets/a54afa12-e91c-4e51-b490-493b6a7fcda5" />

---
[Conway's Game of Life](https://wikipedia.org/wiki/Conway%27s_Game_of_Life) programmed in [Rust](https://rust-lang.org/).

# Demo
https://github.com/user-attachments/assets/754f1e42-7981-404b-893d-dd70ea5f8bad

# Features
- Core engine that produces the next generation of cells from the previous generation
- CLI output for cross-platform usage
- Flexible cell input
- Easy to understand instructions
- Total control over speed of each simulation
- Ability to customize grid size of simulation space
- Error handling
- Blazing fast generation with Rust
- Preloaded grid sizes for convenience

# How to use
1. In the [releases](https://github.com/Polycarbohydrate/FGOL/releases) section, download the binary for your platform (Mac, Linux, Windows).
2. Run the binary and follow the instructions in the terminal.

Alternative 1:
1. Make sure Rust is installed. [Click me to see instructions!](https://rust-lang.org/learn/get-started/)
2. Once Rust is installed [clone](https://docs.github.com/en/repositories/creating-and-managing-repositories/cloning-a-repository) this repository.
3. Open the terminal in the directory of this repository and build and run using the command `cargo run` or `cargo run --release`. (IDEs and some text editors, such as VSCode, will have a terminal that is already loaded into the proper repository directory.)

Alternative 2:
1. Clone this repository.
2. After cloning, the GitHub actions workflow for binary compilation will automatically run.
3. Go into the actions page for the workflow run. Scroll down to the produced artifacts section.
4. Download the desired binary for your platform.

Follow the instructions on the terminal to start your own simulation!

# AI Usage
No AI was used in any part of the program. This includes all files under `src/` (`main.rs`, `inputs.rs`, `cli_display.rs`, `core.rs`).  
All algorithms and program design were made by me (Polycarbohydrate).

# FAQs
Q1: Why is the program a downloadable executable and not a web app or library/crate package?  

A1: I am using pure Rust which makes a web app much more time consuming and difficult then any other platform. I would need to learn `eframe/egui` as well as WASM conversion with `wasm-bindgen`. I am planning to create a web GUI site in the future.  

Q2: When running the executable, my antivirus displays a warning and will not run it. Is the program safe?  

A2: Their are many options to prove authenticity of the binaries. Read the release info to see that the binaries are produced from what is in the repository. If you truely do not want to run the provided binaries, you can follow alternative steps 1 and 2.  
