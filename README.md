# Git Visualizer

A terminal-based Git repository visualizer with a beautiful TUI interface. View and navigate through your Git history with ease.

## Features

- 📊 Timeline view of commits with detailed information
- 👤 Author filtering (press 'a' to filter by author)
- 🌳 Branch selector (press 'b' to switch between branches)
- 🔍 Diff viewer for selected commits
- ⚡ Fast navigation with arrow keys
- 🔎 Quick search with '/' key
- 💾 Efficient caching system
- 🎨 Clean and modern TUI interface

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

1. Navigate to a Git repository:
   ```bash
   cd /path/to/your/repo
   ```

2. Run the visualizer:
   ```bash
   git-visualiser
   ```

### Keyboard Shortcuts

- `↑/↓`: Navigate through commits
- `←/→`: Navigate between panels
- `a`: Toggle author filter
- `b`: Toggle branch selector
- `/`: Start search
- `q`: Quit

## Dependencies

- [ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI library
- [crossterm](https://github.com/crossterm-rs/crossterm) - Terminal manipulation
- [git2](https://github.com/rust-lang/git2-rs) - Git operations
- [serde](https://serde.rs/) - Serialization framework
- [chrono](https://github.com/chronotope/chrono) - Date and time handling

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details. 