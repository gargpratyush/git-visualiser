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
        let mut lines = vec![
            format!("Hash: {}", commit.hash),
            format!("Author: {}", commit.author),
            format!("Date: {}", commit.date),
            String::new(),
            format!("Message:\n{}", commit.message),
            String::new(),
            "Changed Files:".to_string(),
        ];

        if let Some(diff) = &commit.diff {
            // Parse the diff to extract file names and change statistics
            let mut current_file = String::new();
            let mut insertions = 0;
            let mut deletions = 0;
            let mut file_changes = Vec::new();
            let mut in_file_section = false;
            
            for line in diff.lines() {
                if line.starts_with("diff --git") {
                    // Save previous file stats if we have them
                    if !current_file.is_empty() {
                        file_changes.push((current_file.clone(), insertions, deletions));
                        insertions = 0;
                        deletions = 0;
                    }
                    
                    // Extract the new file name
                    if let Some(file_part) = line.split(" b/").nth(1) {
                        current_file = file_part.to_string();
                    }
                    in_file_section = false;
                } else if line.starts_with("@@") {
                    in_file_section = true;
                } else if in_file_section {
                    if line.starts_with("+") && !line.starts_with("+++") {
                        insertions += 1;
                    } else if line.starts_with("-") && !line.starts_with("---") {
                        deletions += 1;
                    }
                }
            }
            
            // Add the last file's stats
            if !current_file.is_empty() {
                file_changes.push((current_file, insertions, deletions));
            }
            
            // If we couldn't parse the diff properly, show a message
            if file_changes.is_empty() {
                lines.push("Unable to parse diff information".to_string());
            } else {
                // Add each file with its change statistics
                for (file, ins, del) in file_changes {
                    let change_type = if ins > 0 && del > 0 {
                        "modified"
                    } else if ins > 0 {
                        "added"
                    } else if del > 0 {
                        "deleted"
                    } else {
                        "renamed"
                    };

                    let status_style = match change_type {
                        "modified" => Style::default().fg(Color::Yellow),
                        "added" => Style::default().fg(Color::Green),
                        "deleted" => Style::default().fg(Color::Red),
                        _ => Style::default().fg(Color::Blue),
                    };

                    lines.push(format!("{:<8} {} (+{}, -{})", 
                        change_type, file, ins, del));
                }
            }
        } else {
            lines.push("No diff available".to_string());
        }

        lines.join("\n")
    } else {
        String::from("No commit selected")
    };

    let paragraph = Paragraph::new(content)
        .block(Block::default().title("Details").borders(Borders::ALL));

    f.render_widget(paragraph, area);
} 