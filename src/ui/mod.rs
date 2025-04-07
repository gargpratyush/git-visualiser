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
            // DEBUG: Print the raw diff to understand its format
            // println!("DEBUG RAW DIFF:\n{}", diff);

            // Parse the diff to extract file names and change statistics
            let mut current_file = String::new();
            let mut old_file = String::new();
            let mut insertions = 0;
            let mut deletions = 0;
            let mut file_changes = Vec::new();
            let mut is_rename = false;
            let mut similarity_index = 0;
            let mut is_in_hunk = false;
            let mut has_content_change = false;
            
            let mut lines_iter = diff.lines().peekable();
            
            while let Some(line) = lines_iter.next() {
                if line.starts_with("diff --git") {
                    // Save previous file stats if we have them
                    if !current_file.is_empty() {
                        let change_type = if is_rename {
                            if similarity_index == 100 {
                                "renamed"
                            } else {
                                "renamed+modified"
                            }
                        } else if insertions > 0 || deletions > 0 || has_content_change {
                            "modified"
                        } else {
                            "unchanged"
                        };
                        
                        file_changes.push((current_file.clone(), old_file.clone(), insertions, deletions, change_type.to_string()));
                        current_file.clear();
                        old_file.clear();
                        insertions = 0;
                        deletions = 0;
                        is_rename = false;
                        is_in_hunk = false;
                        similarity_index = 0;
                        has_content_change = false;
                    }
                    
                    // Extract the file names from diff --git a/old b/new
                    let parts: Vec<&str> = line.split(" ").collect();
                    if parts.len() >= 4 {
                        old_file = parts[2].trim_start_matches("a/").to_string();
                        current_file = parts[3].trim_start_matches("b/").to_string();
                    }
                    
                    // Look ahead for rename markers
                    let mut peek_iter = lines_iter.clone();
                    while let Some(peek_line) = peek_iter.next() {
                        if peek_line.starts_with("similarity index ") {
                            // Extract similarity percentage
                            if let Some(percent_str) = peek_line.strip_prefix("similarity index ") {
                                if percent_str.ends_with('%') {
                                    if let Ok(percent) = percent_str.trim_end_matches('%').parse::<usize>() {
                                        similarity_index = percent;
                                    }
                                }
                            }
                        } else if peek_line.starts_with("rename from") || peek_line.starts_with("rename to") {
                            is_rename = true;
                        } else if peek_line.starts_with("--- ") || peek_line.starts_with("+++ ") {
                            has_content_change = true;
                        } else if peek_line.starts_with("@@") {
                            has_content_change = true;
                            break;
                        } else if peek_line.starts_with("diff --git") {
                            // Reached next file
                            break;
                        }
                    }
                    
                } else if line.starts_with("similarity index ") {
                    // Extract similarity percentage
                    if let Some(percent_str) = line.strip_prefix("similarity index ") {
                        if percent_str.ends_with('%') {
                            if let Ok(percent) = percent_str.trim_end_matches('%').parse::<usize>() {
                                similarity_index = percent;
                            }
                        }
                    }
                } else if line.starts_with("rename from") {
                    is_rename = true;
                    old_file = line.trim_start_matches("rename from ").to_string();
                } else if line.starts_with("rename to") {
                    is_rename = true;
                    current_file = line.trim_start_matches("rename to ").to_string();
                } else if line.starts_with("--- ") || line.starts_with("+++ ") {
                    has_content_change = true;
                } else if line.starts_with("@@") {
                    is_in_hunk = true;
                    has_content_change = true;
                    // Hunk header - continue to process the content
                    continue;
                } else if is_in_hunk && line.starts_with("+") && !line.starts_with("+++") {
                    insertions += 1;
                } else if is_in_hunk && line.starts_with("-") && !line.starts_with("---") {
                    deletions += 1;
                }
            }
            
            // Add the last file's stats
            if !current_file.is_empty() {
                let change_type = if is_rename {
                    if similarity_index == 100 {
                        "renamed"
                    } else {
                        "renamed+modified"
                    }
                } else if insertions > 0 || deletions > 0 || has_content_change {
                    "modified"
                } else {
                    "unchanged"
                };
                
                file_changes.push((current_file, old_file, insertions, deletions, change_type.to_string()));
            }
            
            // If we couldn't parse the diff properly, show a message
            if file_changes.is_empty() {
                lines.push("No files changed".to_string());
            } else {
                // Add each file with its change statistics
                for (new_file, old_file, ins, del, change_type) in file_changes {
                    let status = match change_type.as_str() {
                        "renamed" => format!("renamed {} →", old_file),
                        "renamed+modified" => format!("renamed+mod {} →", old_file),
                        "modified" => "modified".to_string(),
                        _ => "unchanged".to_string(),
                    };
                    
                    // Show proper line change counts for each type
                    let changes = if change_type == "renamed" && ins == 0 && del == 0 {
                        "(renamed only)".to_string()
                    } else {
                        format!("(+{}, -{})", ins, del)
                    };
                    
                    lines.push(format!("{:<30} {:<40} {}", 
                        status,
                        new_file,
                        changes
                    ));
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

fn parse_range(range: &str) -> (usize, usize) {
    let parts: Vec<&str> = range.split(',').collect();
    match parts.as_slice() {
        [start, len] => (
            start.parse().unwrap_or(0),
            len.parse().unwrap_or(0)
        ),
        [single] => (
            single.parse().unwrap_or(0),
            1
        ),
        _ => (0, 0)
    }
} 