use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Clear},
    Frame, Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, time::Duration};
use std::collections::VecDeque;
use crate::models::CommitInfo;

pub struct App {
    pub commits: VecDeque<CommitInfo>,
    pub selected_index: usize,
    pub author_filter: Option<String>,
    pub current_branch: String,
    pub branches: Vec<String>,
    pub search_mode: bool,
    pub search_query: String,
    pub show_author_filter: bool,
    pub show_branch_selector: bool,
    pub branch_selector_index: usize,
}

impl App {
    pub fn new() -> Self {
        App {
            commits: VecDeque::new(),
            selected_index: 0,
            author_filter: None,
            current_branch: String::from("main"),
            branches: Vec::new(),
            search_mode: false,
            search_query: String::new(),
            show_author_filter: false,
            show_branch_selector: false,
            branch_selector_index: 0,
        }
    }

    pub fn toggle_author_filter(&mut self) {
        self.show_author_filter = !self.show_author_filter;
        if self.show_author_filter {
            // TODO: Implement author filter selection
        }
    }

    pub fn toggle_branch_selector(&mut self) {
        self.show_branch_selector = !self.show_branch_selector;
        if self.show_branch_selector {
            // Find the current branch in the list
            self.branch_selector_index = self.branches.iter()
                .position(|b| b == &self.current_branch)
                .unwrap_or(0);
        }
    }

    pub fn select_branch(&mut self, index: usize) -> bool {
        if index < self.branches.len() {
            self.current_branch = self.branches[index].clone();
            self.branch_selector_index = index;
            true
        } else {
            false
        }
    }

    pub fn navigate_branch_selector(&mut self, direction: i32) {
        let new_index = self.branch_selector_index as i32 + direction;
        if new_index >= 0 && new_index < self.branches.len() as i32 {
            self.branch_selector_index = new_index as usize;
        }
    }

    pub fn start_search(&mut self) {
        self.search_mode = true;
    }

    pub fn navigate_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn navigate_down(&mut self) {
        if self.selected_index < self.commits.len().saturating_sub(1) {
            self.selected_index += 1;
        }
    }

    pub fn navigate_left(&mut self) {
        // TODO: Implement left navigation
    }

    pub fn navigate_right(&mut self) {
        // TODO: Implement right navigation
    }
}

pub fn draw_ui(f: &mut Frame, app: &App) {
    let size = f.size();

    if app.show_branch_selector {
        draw_branch_selector(f, app, size);
        return;
    }

    if app.show_author_filter {
        draw_author_filter(f, app, size);
        return;
    }

    // Create the main layout
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .split(size);

    // Draw the commit list
    draw_commit_list(f, app, chunks[0]);

    // Draw the commit details
    draw_commit_details(f, app, chunks[1]);
}

fn draw_branch_selector(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .branches
        .iter()
        .enumerate()
        .map(|(i, branch)| {
            let style = if i == app.branch_selector_index {
                Style::default().bg(Color::Blue)
            } else if branch == &app.current_branch {
                Style::default().fg(Color::Green)
            } else {
                Style::default()
            };

            let prefix = if branch.contains('/') {
                "🌐 " // Remote branch
            } else {
                "🌿 " // Local branch
            };

            ListItem::new(Line::from(vec![
                Span::styled(
                    format!("{}{}", prefix, branch),
                    style,
                ),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().title("Select Branch (↑/↓ to navigate, Enter to select, Esc to cancel)").borders(Borders::ALL));

    f.render_widget(list, area);
}

fn draw_author_filter(f: &mut Frame, app: &App, area: Rect) {
    // TODO: Implement author filter UI
    let paragraph = Paragraph::new("Author filter (not implemented yet)")
        .block(Block::default().title("Author Filter").borders(Borders::ALL));

    f.render_widget(paragraph, area);
}

fn draw_commit_list(f: &mut Frame, app: &App, area: Rect) {
    if app.commits.is_empty() {
        let empty_message = Paragraph::new("No commits found in the repository.")
            .block(Block::default().title("Commits").borders(Borders::ALL));
        f.render_widget(empty_message, area);
        return;
    }

    let items: Vec<ListItem> = app
        .commits
        .iter()
        .enumerate()
        .map(|(i, commit)| {
            let style = if i == app.selected_index {
                Style::default().bg(Color::Blue)
            } else {
                Style::default()
            };

            ListItem::new(Line::from(vec![
                Span::styled(
                    format!("{} {}", commit.hash, commit.message),
                    style,
                ),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().title(format!("Commits ({})", app.current_branch)).borders(Borders::ALL));

    f.render_widget(list, area);
}

fn draw_commit_details(f: &mut Frame, app: &App, area: Rect) {
    if app.commits.is_empty() {
        let empty_message = Paragraph::new("No commit selected.")
            .block(Block::default().title("Details").borders(Borders::ALL));
        f.render_widget(empty_message, area);
        return;
    }

    let commit = app.commits.get(app.selected_index);
    
    let content = if let Some(commit) = commit {
        format!(
            "Hash: {}\nAuthor: {}\nDate: {}\n\nMessage:\n{}\n\nDiff:\n{}",
            commit.hash,
            commit.author,
            commit.date,
            commit.message,
            commit.diff.as_deref().unwrap_or("No diff available")
        )
    } else {
        String::from("No commit selected")
    };

    let paragraph = Paragraph::new(content)
        .block(Block::default().title("Details").borders(Borders::ALL));

    f.render_widget(paragraph, area);
} 