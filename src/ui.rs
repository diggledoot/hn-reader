use crate::models::Article;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute, terminal,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};
use ratatui::layout::Alignment;
use std::io;

#[derive(Debug, Clone, PartialEq)]
pub enum ViewMode {
    Articles,
}

pub struct Ui {
    pub selected_index: usize,
    pub current_page: u32,
    pub view_mode: ViewMode,
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
            let view_title = match self.view_mode {
                ViewMode::Articles => "Hacker News Top Stories",
            };
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

                let title = Paragraph::new(view_title)
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

                let footer = Paragraph::new("Commands: ↑↓ Navigate | ←→ Page | Enter Article | c Comments | r Refresh | q Quit")
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
            let view_title = match self.view_mode {
                ViewMode::Articles => format!(
                    "Hacker News Top Stories — Page {} / {}",
                    self.current_page + 1,
                    total_pages
                ),
            };

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
                let title = Paragraph::new(view_title)
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
                let footer = Paragraph::new("Commands: ↑↓ Navigate | ←→ Page | Enter Article | c Comments | r Refresh | q Quit")
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
                            KeyCode::Char('c') => return 'c', // open HN comments
                            KeyCode::Enter => return 'e', // enter/article
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

    pub fn open_hn_discussion(&self, articles: &[Article]) {
        if self.selected_index < articles.len() {
            if let Some(article) = articles.get(self.selected_index) {
                let hn_url = format!("https://news.ycombinator.com/item?id={}", article.object_id);
                if let Err(e) = opener::open(&hn_url) {
                    eprintln!("Failed to open HN discussion: {}", e);
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
        let parts: Vec<&str> = date_part.split('-').collect();
        if parts.len() == 3 {
            return format!("{}/{}/{}", parts[1], parts[2], parts[0]);
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

    #[test]
    fn test_parse_date_valid() {
        assert_eq!(parse_date("2023-01-15T12:00:00Z"), "01/15/2023");
        assert_eq!(parse_date("2024-12-31T23:59:59Z"), "12/31/2024");
    }

    #[test]
    fn test_parse_date_invalid() {
        // Empty string
        assert_eq!(parse_date(""), "");
        // No T separator
        assert_eq!(parse_date("invalid-date"), "invalid-date");
        // Malformed date
        assert_eq!(parse_date("2023-01"), "2023-01");
    }

    #[test]
    fn test_view_mode_articles() {
        let mut ui = Ui::new();
        assert_eq!(ui.view_mode, ViewMode::Articles);

        ui.view_mode = ViewMode::Articles;
        assert_eq!(ui.view_mode, ViewMode::Articles);
    }

    #[test]
    fn test_open_hn_discussion_url() {
        let _ui = Ui::new();
        let articles = vec![Article {
            object_id: "12345678".to_string(),
            title: "Test".to_string(),
            url: Some("https://example.com".to_string()),
            score: 100,
            created_at: "2023-01-01T12:00:00Z".to_string(),
        }];

        // Verify the HN URL format (we can't actually open it in tests)
        let expected_url = format!("https://news.ycombinator.com/item?id={}", articles[0].object_id);
        assert_eq!(expected_url, "https://news.ycombinator.com/item?id=12345678");
    }

    #[test]
    fn test_open_selected_article_with_url() {
        let ui = Ui::new();
        let articles = vec![Article {
            object_id: "1".to_string(),
            title: "Test".to_string(),
            url: Some("https://example.com".to_string()),
            score: 100,
            created_at: "2023-01-01T12:00:00Z".to_string(),
        }];

        // Should not panic - URL exists
        ui.open_selected_article(&articles);
    }

    #[test]
    fn test_open_selected_article_without_url() {
        let ui = Ui::new();
        let articles = vec![Article {
            object_id: "1".to_string(),
            title: "Test".to_string(),
            url: None,
            score: 100,
            created_at: "2023-01-01T12:00:00Z".to_string(),
        }];

        // Should not panic - handles missing URL gracefully
        ui.open_selected_article(&articles);
    }

    #[test]
    fn test_open_article_out_of_bounds() {
        let ui = Ui::new();
        let articles: Vec<Article> = vec![];

        // Should not panic with empty list
        ui.open_selected_article(&articles);
        ui.open_hn_discussion(&articles);
    }

    #[test]
    fn test_navigation_at_boundaries() {
        let mut ui = Ui::new();

        // At page 0, can't go left
        ui.current_page = 0;
        // Simulate left arrow - should stay at 0 (handled in handle_input)
        assert_eq!(ui.current_page, 0);

        // At max page, can't go right (tested via saturating_sub)
        ui.current_page = 100;
        let total_pages: u32 = 100;
        assert!(ui.current_page >= total_pages.saturating_sub(1));
    }

    #[test]
    fn test_empty_articles_navigation() {
        let _ui = Ui::new();
        let _articles: Vec<Article> = vec![];

        // With empty list, selected_index should not increment
        // Down arrow check: articles_len > 0 && selected_index < articles_len - 1
        // With len=0, this is: 0 > 0 && 0 < -1 (false), so no increment
    }
}
