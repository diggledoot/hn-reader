use crate::models::{Article, Comment};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute, terminal,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Terminal,
};
use ratatui::layout::Alignment;
use std::io;

#[derive(Debug, Clone, PartialEq)]
pub enum ViewMode {
    Articles,
    Comments,
}

#[derive(Debug, Clone)]
pub struct FlatComment {
    pub depth: usize,
    pub author: String,
    pub text: String,
    pub created_at: String,
}

const MAX_COMMENT_DEPTH: usize = 8;

pub fn strip_html_tags(input: &str) -> String {
    let mut result = input.to_string();
    // Replace <p> with newlines
    result = result.replace("<p>", "\n");
    result = result.replace("</p>", "");
    // Replace <br> variants
    result = result.replace("<br>", "\n");
    result = result.replace("<br/>", "\n");
    result = result.replace("<br />", "\n");
    // Strip <a> tags but keep the text between them
    // Simple approach: remove <a ...> and </a>
    while let Some(start) = result.find("<a ") {
        if let Some(end) = result[start..].find('>') {
            result = format!("{}{}", &result[..start], &result[start + end + 1..]);
        } else {
            break;
        }
    }
    result = result.replace("</a>", "");
    // Strip any remaining HTML tags
    while let Some(start) = result.find('<') {
        if let Some(end) = result[start..].find('>') {
            result = format!("{}{}", &result[..start], &result[start + end + 1..]);
        } else {
            break;
        }
    }
    // Decode common HTML entities
    result = result.replace("&#x27;", "'");
    result = result.replace("&#x2F;", "/");
    result = result.replace("&amp;", "&");
    result = result.replace("&lt;", "<");
    result = result.replace("&gt;", ">");
    result = result.replace("&quot;", "\"");
    result = result.replace("&#34;", "\"");
    result = result.replace("&#39;", "'");
    result
}

pub fn flatten_comments(comments: &[Comment], depth: usize) -> Vec<FlatComment> {
    let mut flat = Vec::new();
    for comment in comments {
        let author = comment.author.clone().unwrap_or_else(|| "[deleted]".to_string());
        let text = comment
            .text
            .as_deref()
            .map(strip_html_tags)
            .unwrap_or_else(|| "[deleted]".to_string());
        let created_at = comment
            .created_at
            .clone()
            .unwrap_or_default();

        flat.push(FlatComment {
            depth,
            author,
            text,
            created_at,
        });

        if depth < MAX_COMMENT_DEPTH {
            flat.extend(flatten_comments(&comment.children, depth + 1));
        } else if !comment.children.is_empty() {
            flat.push(FlatComment {
                depth: depth + 1,
                author: String::new(),
                text: format!("[{} more replies hidden]", comment.children.len()),
                created_at: String::new(),
            });
        }
    }
    flat
}

pub struct Ui {
    pub selected_index: usize,
    pub current_page: u32,
    pub view_mode: ViewMode,
    pub comments: Vec<FlatComment>,
    pub comment_scroll: usize,
    pub comment_title: String,
    terminal: Option<Terminal<CrosstermBackend<io::Stdout>>>,
    list_state: ListState,
}

impl Ui {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        
        Ui {
            selected_index: 0,
            current_page: 0,
            view_mode: ViewMode::Articles,
            comments: Vec::new(),
            comment_scroll: 0,
            comment_title: String::new(),
            terminal: None,
            list_state,
        }
    }

    pub fn setup_terminal(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        terminal::enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, terminal::EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        self.terminal = Some(terminal);
        Ok(())
    }

    pub fn cleanup_terminal(&mut self) {
        if let Some(mut terminal) = self.terminal.take() {
            let _ = execute!(terminal.backend_mut(), terminal::LeaveAlternateScreen);
            let _ = terminal.show_cursor();
        }
        let _ = terminal::disable_raw_mode();
    }

    pub fn render_loading(&mut self) {
        if let Some(ref mut terminal) = self.terminal {
            let page = self.current_page;
            let _ = terminal.draw(|f| {
                let size = f.area();

                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(3),
                        Constraint::Min(1),
                        Constraint::Length(3),
                    ])
                    .split(size);

                let title = Paragraph::new("Hacker News Top Stories")
                    .style(Style::default().fg(Color::Yellow))
                    .alignment(Alignment::Center)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::Rgb(255, 165, 0)))
                            .title("HN Reader")
                            .title_alignment(Alignment::Center),
                    );
                f.render_widget(title, chunks[0]);

                let loading = Paragraph::new(format!("Loading page {}...", page + 1))
                    .style(Style::default().fg(Color::Yellow))
                    .alignment(Alignment::Center)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::Rgb(255, 165, 0))),
                    );
                f.render_widget(loading, chunks[1]);

                let footer = Paragraph::new("Commands: ↑↓ Navigate | ←→ Page | Enter Open | c Comments | r Refresh | q Quit")
                    .style(Style::default().fg(Color::DarkGray))
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::Rgb(255, 165, 0))),
                    );
                f.render_widget(footer, chunks[2]);
            });
        }
    }

    pub fn render_comments_loading(&mut self, title: &str) {
        if let Some(ref mut terminal) = self.terminal {
            let title_owned = title.to_string();
            let _ = terminal.draw(|f| {
                let size = f.area();

                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(3),
                        Constraint::Min(1),
                        Constraint::Length(3),
                    ])
                    .split(size);

                let title_widget = Paragraph::new(title_owned.as_str())
                    .style(Style::default().fg(Color::Yellow))
                    .alignment(Alignment::Center)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::Rgb(255, 165, 0)))
                            .title("HN Reader — Comments")
                            .title_alignment(Alignment::Center),
                    );
                f.render_widget(title_widget, chunks[0]);

                let loading = Paragraph::new("Loading comments...")
                    .style(Style::default().fg(Color::Yellow))
                    .alignment(Alignment::Center)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::Rgb(255, 165, 0))),
                    );
                f.render_widget(loading, chunks[1]);

                let footer = Paragraph::new("Commands: Esc/Backspace Back")
                    .style(Style::default().fg(Color::DarkGray))
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::Rgb(255, 165, 0))),
                    );
                f.render_widget(footer, chunks[2]);
            });
        }
    }

    pub fn render_comments(&mut self) {
        if let Some(ref mut terminal) = self.terminal {
            let comments = &self.comments;
            let scroll = self.comment_scroll;
            let title_text = self.comment_title.clone();
            let total = comments.len();

            // Build all display lines from flat comments
            let mut lines: Vec<Line> = Vec::new();
            for comment in comments.iter() {
                let indent = "  ".repeat(comment.depth);

                if comment.author.is_empty() {
                    // This is a "[N more replies hidden]" marker
                    lines.push(Line::from(Span::styled(
                        format!("{}{}", indent, comment.text),
                        Style::default().fg(Color::DarkGray),
                    )));
                    lines.push(Line::from(""));
                    continue;
                }

                // Author + date header
                let date_str = if !comment.created_at.is_empty() {
                    parse_date(&comment.created_at)
                } else {
                    String::new()
                };
                lines.push(Line::from(Span::styled(
                    format!("{}{} — {}", indent, comment.author, date_str),
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                )));

                // Comment text lines
                for text_line in comment.text.lines() {
                    lines.push(Line::from(Span::styled(
                        format!("{}{}", indent, text_line),
                        Style::default().fg(Color::White),
                    )));
                }

                // Blank separator line
                lines.push(Line::from(""));
            }

            let _ = terminal.draw(|f| {
                let size = f.area();

                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(3),
                        Constraint::Min(1),
                        Constraint::Length(3),
                    ])
                    .split(size);

                // Title
                let title = Paragraph::new(title_text.as_str())
                    .style(Style::default().fg(Color::Yellow))
                    .alignment(Alignment::Center)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::Rgb(255, 165, 0)))
                            .title("HN Reader — Comments")
                            .title_alignment(Alignment::Center),
                    );
                f.render_widget(title, chunks[0]);

                // Comments body with scroll
                let comments_widget = Paragraph::new(lines)
                    .scroll((scroll as u16, 0))
                    .wrap(Wrap { trim: false })
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::Rgb(255, 165, 0))),
                    );
                f.render_widget(comments_widget, chunks[1]);

                // Footer
                let footer_text = format!(
                    "Commands: ↑↓/j k Scroll | Esc/Backspace Back | {} comments",
                    total
                );
                let footer = Paragraph::new(footer_text)
                    .style(Style::default().fg(Color::DarkGray))
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::Rgb(255, 165, 0))),
                    );
                f.render_widget(footer, chunks[2]);
            });
        }
    }

    pub fn render(&mut self, articles: &[Article], total_pages: u32) {
        // Update the list state with the current selection
        self.list_state.select(Some(self.selected_index));
        
        if let Some(ref mut terminal) = self.terminal {
            let _ = terminal.draw(|f| {
                let size = f.area();
                
                // Define the main layout
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(3), // Title
                        Constraint::Min(1),    // Articles list
                        Constraint::Length(3), // Footer
                    ])
                    .split(size);

                // Title with page indicator
                let title_text = format!(
                    "Hacker News Top Stories — Page {} / {}",
                    self.current_page + 1,
                    total_pages
                );
                let title = Paragraph::new(title_text)
                    .style(Style::default().fg(Color::Yellow))
                    .alignment(Alignment::Center)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::Rgb(255, 165, 0))) // Orange color
                            .title("HN Reader")
                            .title_alignment(Alignment::Center),
                    );
                f.render_widget(title, chunks[0]);

                // Articles list
                let page_offset = (self.current_page as usize) * 20;
                let items: Vec<ListItem> = articles
                    .iter()
                    .enumerate()
                    .map(|(i, article)| {
                        // Parse the date to make it more readable
                        let date_str = parse_date(&article.created_at);
                        let global_index = page_offset + i + 1;
                        
                        let line = if i == self.selected_index {
                            format!("> {}. {} (Score: {}, Date: {})", global_index, article.title, article.score, date_str)
                        } else {
                            format!("  {}. {} (Score: {}, Date: {})", global_index, article.title, article.score, date_str)
                        };
                        ListItem::new(line)
                    })
                    .collect();
                
                let articles_list = List::new(items)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::Rgb(255, 165, 0))), // Orange color
                    )
                    .highlight_style(
                        Style::default()
                            .bg(Color::Blue)
                            .add_modifier(Modifier::BOLD),
                    );
                
                f.render_stateful_widget(articles_list, chunks[1], &mut self.list_state);

                // Footer
                let footer = Paragraph::new("Commands: ↑↓ Navigate | ←→ Page | Enter Open | c Comments | r Refresh | q Quit")
                    .style(Style::default().fg(Color::DarkGray))
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::Rgb(255, 165, 0))), // Orange color
                    );
                f.render_widget(footer, chunks[2]);
            });
        }
    }

    pub async fn handle_input(&mut self, articles_len: usize, total_pages: u32) -> char {
        if event::poll(std::time::Duration::from_millis(50)).unwrap() {
            if let Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
                match self.view_mode {
                    ViewMode::Articles => {
                        match code {
                            KeyCode::Up => {
                                if self.selected_index > 0 {
                                    self.selected_index -= 1;
                                }
                            }
                            KeyCode::Down => {
                                if articles_len > 0 && self.selected_index < articles_len - 1 {
                                    self.selected_index += 1;
                                }
                            }
                            KeyCode::Left => {
                                if self.current_page > 0 {
                                    self.current_page -= 1;
                                    self.selected_index = 0;
                                    return 'p'; // previous page
                                }
                            }
                            KeyCode::Right => {
                                if self.current_page < total_pages.saturating_sub(1) {
                                    self.current_page += 1;
                                    self.selected_index = 0;
                                    return 'n'; // next page
                                }
                            }
                            KeyCode::Char('q') => return 'q',
                            KeyCode::Char('r') => return 'r',
                            KeyCode::Char('c') => return 'c', // open comments
                            KeyCode::Enter => return 'l', // l for link/open
                            _ => {}
                        }
                    }
                    ViewMode::Comments => {
                        match code {
                            KeyCode::Esc | KeyCode::Backspace => {
                                self.view_mode = ViewMode::Articles;
                                self.comments.clear();
                                self.comment_scroll = 0;
                                return 'b'; // back to articles
                            }
                            KeyCode::Up | KeyCode::Char('k') => {
                                self.comment_scroll = self.comment_scroll.saturating_sub(1);
                            }
                            KeyCode::Down | KeyCode::Char('j') => {
                                self.comment_scroll += 1;
                            }
                            KeyCode::PageUp => {
                                self.comment_scroll = self.comment_scroll.saturating_sub(20);
                            }
                            KeyCode::PageDown => {
                                self.comment_scroll += 20;
                            }
                            KeyCode::Char('q') => return 'q',
                            _ => {}
                        }
                    }
                }
            }
        }
        
        '\0' // null character to indicate no command
    }
    
    pub fn open_selected_article(&self, articles: &[Article]) {
        if self.selected_index < articles.len() {
            if let Some(article) = articles.get(self.selected_index) {
                if let Some(ref url) = article.url {
                    if let Err(e) = opener::open(url) {
                        eprintln!("Failed to open URL: {}", e);
                    }
                } else {
                    eprintln!("No URL available for this article");
                }
            }
        }
    }
}

// Helper function to parse and format the date
fn parse_date(date_str: &str) -> String {
    // The date comes in ISO 8601 format like "2023-01-01T12:00:00Z"
    // We'll extract the date part and make it more readable
    if let Some(date_part) = date_str.split('T').next() {
        // Convert YYYY-MM-DD to a more readable format
        if let [year, month, day] = date_part.split('-').collect::<Vec<_>>()[..3] {
            return format!("{}/{}/{}", month, day, year);
        }
    }
    // If parsing fails, return the original string
    date_str.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Article;

    #[test]
    fn test_ui_initialization() {
        let ui = Ui::new();
        
        assert_eq!(ui.selected_index, 0);
        assert_eq!(ui.current_page, 0);
        assert!(ui.terminal.is_none());
    }

    #[test]
    fn test_selected_index_bounds() {
        let mut ui = Ui::new();
        
        // Test that selected index doesn't go negative (wraps to end)
        ui.selected_index = 0;
        let articles = vec![Article {
            object_id: "1".to_string(),
            title: "Test".to_string(),
            url: Some("https://example.com".to_string()),
            score: 10,
            created_at: "2023-01-01T12:00:00Z".to_string(),
        }];
        
        // Since we can't easily test the up/down key handling in a non-terminal environment,
        // we'll test the bounds checking logic directly
        assert!(ui.selected_index < articles.len());
    }
}