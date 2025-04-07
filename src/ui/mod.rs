use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use std::collections::VecDeque;
use crate::models::CommitInfo;

pub struct App {
    pub commits: VecDeque<CommitInfo>,
    pub selected_index: usize,
    pub current_branch: String,
    pub branches: Vec<String>,
    pub show_author_filter: bool,
    pub show_branch_selector: bool,
    pub branch_selector_index: usize,
}

impl App {
    pub fn toggle_author_filter(&mut self) {
        self.show_author_filter = !self.show_author_filter;
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

fn draw_author_filter(f: &mut Frame, _app: &App, area: Rect) {
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
            // Simplified approach - just extract file names from diff
            let mut current_file = String::new();
            let mut old_file = String::new();
            let mut is_rename = false;
            let mut is_new_file = false;
            let mut is_deleted_file = false;
            let mut file_changes = Vec::new();
            
            // Process each line in the diff to extract changed files
            for line in diff.lines() {
                if line.starts_with("diff --git") {
                    // Save previous file info
                    if !current_file.is_empty() {
                        let file_type = if is_new_file {
                            "added"
                        } else if is_deleted_file {
                            "deleted"
                        } else if is_rename {
                            "renamed"
                        } else {
                            "modified"
                        };
                        
                        file_changes.push((current_file.clone(), old_file.clone(), file_type.to_string()));
                    }
                    
                    // Reset for new file
                    current_file = String::new();
                    old_file = String::new();
                    is_rename = false;
                    is_new_file = false;
                    is_deleted_file = false;
                    
                    // Extract file names from diff header
                    let parts: Vec<&str> = line.split(' ').collect();
                    if parts.len() >= 4 {
                        old_file = parts[2].trim_start_matches("a/").to_string();
                        current_file = parts[3].trim_start_matches("b/").to_string();
                    }
                }
                else if line.starts_with("new file mode") {
                    is_new_file = true;
                }
                else if line.starts_with("deleted file mode") {
                    is_deleted_file = true;
                }
                else if line.starts_with("rename from") {
                    is_rename = true;
                    old_file = line.trim_start_matches("rename from ").to_string();
                }
                else if line.starts_with("rename to") {
                    is_rename = true;
                    current_file = line.trim_start_matches("rename to ").to_string();
                }
            }
            
            // Add the last file
            if !current_file.is_empty() {
                let file_type = if is_new_file {
                    "added"
                } else if is_deleted_file {
                    "deleted"
                } else if is_rename {
                    "renamed"
                } else {
                    "modified"
                };
                
                file_changes.push((current_file, old_file, file_type.to_string()));
            }
            
            // Display the files and their changes
            if file_changes.is_empty() {
                lines.push("No files changed".to_string());
            } else {
                for (file, old_path, change_type) in &file_changes {
                    // Format the status based on change type
                    let status = match change_type.as_str() {
                        "added" => "added".to_string(),
                        "deleted" => "deleted".to_string(),
                        "renamed" => format!("renamed {} →", old_path),
                        _ => "modified".to_string(),
                    };
                    
                    lines.push(format!("{:<30} {}", status, file));
                }
                
                // Add a summary line with just the count of files
                lines.push(String::new());
                lines.push(format!("Total: {} files changed", file_changes.len()));
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