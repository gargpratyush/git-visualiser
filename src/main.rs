mod ui;
mod git;
mod cache;
mod models;

use anyhow::{Result, Context};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;
use std::path::PathBuf;
use std::collections::VecDeque;
use crate::ui::App;
use crate::git::GitManager;
use crate::cache::Cache;
use crate::models::CommitInfo;

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode().context("Failed to enable raw mode")?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
        .context("Failed to enter alternate screen")?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).context("Failed to create terminal")?;

    // Initialize Git manager and cache
    let current_dir = std::env::current_dir().context("Failed to get current directory")?;
    
    // Check if we're in a Git repository
    if !current_dir.join(".git").exists() {
        println!("Error: Not a Git repository. Please run this application from a Git repository.");
        return Ok(());
    }
    
    let git_manager = match GitManager::new(&current_dir) {
        Ok(manager) => manager,
        Err(e) => {
            println!("Error: Failed to open Git repository: {}", e);
            return Ok(());
        }
    };
    
    let mut cache = Cache::new();
    
    // Get current branch
    let branches = match git_manager.get_branches() {
        Ok(branches) => branches,
        Err(e) => {
            println!("Error: Failed to get branches: {}", e);
            return Ok(());
        }
    };
    
    // Try to find a valid branch to use
    let current_branch = if branches.is_empty() {
        // Try 'main' first, then 'master' as fallback
        if git_manager.branch_exists("main") {
            "main".to_string()
        } else if git_manager.branch_exists("master") {
            "master".to_string()
        } else {
            println!("Error: No valid branches found in the repository.");
            return Ok(());
        }
    } else {
        branches[0].clone()
    };
    
    println!("Using branch: {}", current_branch);
    
    // Get authors
    let authors = match git_manager.get_authors() {
        Ok(authors) => authors,
        Err(e) => {
            println!("Error: Failed to get authors: {}", e);
            return Ok(());
        }
    };
    
    // Get commits
    let commits = match git_manager.get_commits(&current_branch) {
        Ok(commits) => {
            if commits.is_empty() {
                println!("No commits found in the repository.");
                return Ok(());
            }
            commits
        },
        Err(e) => {
            println!("Error: Failed to get commits: {}", e);
            return Ok(());
        }
    };
    
    // Create app state
    let mut app = App {
        commits: VecDeque::from(commits),
        selected_index: 0,
        author_filter: None,
        current_branch,
        branches,
        search_mode: false,
        search_query: String::new(),
        show_author_filter: false,
        show_branch_selector: false,
    };

    // Main loop
    loop {
        terminal.draw(|f| ui::draw_ui(f, &app)).context("Failed to draw UI")?;

        // Handle input
        if event::poll(std::time::Duration::from_millis(100)).context("Failed to poll for events")? {
            if let Event::Key(key) = event::read().context("Failed to read event")? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('a') => app.toggle_author_filter(),
                    KeyCode::Char('b') => app.toggle_branch_selector(),
                    KeyCode::Char('/') => app.start_search(),
                    KeyCode::Up => app.navigate_up(),
                    KeyCode::Down => app.navigate_down(),
                    KeyCode::Left => app.navigate_left(),
                    KeyCode::Right => app.navigate_right(),
                    _ => {}
                }
            }
        }
    }

    // Cleanup
    disable_raw_mode().context("Failed to disable raw mode")?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    ).context("Failed to leave alternate screen")?;
    terminal.show_cursor().context("Failed to show cursor")?;

    Ok(())
}
