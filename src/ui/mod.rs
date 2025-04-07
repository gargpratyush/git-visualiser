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
    pub author_filter: Option<String>,
    pub current_branch: String,
    pub branches: Vec<String>,
    pub search_mode: bool,
    pub search_query: String,
    pub show_author_filter: bool,
    pub show_branch_selector: bool,
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
            // TODO: Implement branch selection
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
        .block(Block::default().title("Commits").borders(Borders::ALL));

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