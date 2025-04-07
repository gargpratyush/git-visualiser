# Git Visualizer

Better than git log.
I am not a fan of the user-interface of the command git-log. For people like us who like to live inside the terminal, `git log` makes us have second thoughts. Hence, I created Git Visualiser, a better TUI so that we don't need to use git log command anymore.

## Features

1. View what has changed in the commit.
   ![image](https://github.com/user-attachments/assets/6914d43f-7b82-465a-b8d1-3423f4fec595)

2. Also tells you if the file has been added/deleted or modified.
   ![image](https://github.com/user-attachments/assets/0caa0061-1fe9-4fd0-b8d7-87b4864875ef)

3. Navigate between different branches (press `b`).
   ![image](https://github.com/user-attachments/assets/9021dc26-1f2d-41ca-930c-11094e8f197b)


## Installation

1. Make sure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/)
2. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/git-visualiser.git
   cd git-visualiser
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

1. Navigate to a Git repository where you want to view commit history:
   ```bash
   cd /path/to/your/repo
   ```

2. Run the visualizer:
   ```bash
   ./{path-to-git-visualiser-repository}/target/release/git-visualiser
   ```

### Keyboard Shortcuts

- `↑/↓`: Navigate through commits
- `a`: Toggle author filter
- `b`: Toggle branch selector
- `q`: Quit

## Dependencies

- [ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI library
- [crossterm](https://github.com/crossterm-rs/crossterm) - Terminal manipulation
- [git2](https://github.com/rust-lang/git2-rs) - Git operations
- [serde](https://serde.rs/) - Serialization framework
- [chrono](https://github.com/chronotope/chrono) - Date and time handling

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
