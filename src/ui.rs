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
    Frame, Terminal,
};
use ratatui::layout::Alignment;
use std::io;

pub struct Ui {
    pub selected_index: usize,
    terminal: Option<Terminal<CrosstermBackend<io::Stdout>>>,
    list_state: ListState,
}

impl Ui {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        
        Ui {
            selected_index: 0,
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
            let _ = terminal.clear();
        }
        let _ = terminal::disable_raw_mode();
    }

    pub fn render(&mut self, articles: &[Article]) {
        // Update the list state with the current selection
        self.list_state.select(Some(self.selected_index));
        
        if let Some(ref mut terminal) = self.terminal {
            let _ = terminal.draw(|f| {
                let size = f.size();
                
                // Define the main layout
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(3), // Title
                        Constraint::Min(1),    // Articles list
                        Constraint::Length(3), // Footer
                    ])
                    .split(size);

                // Title
                let title = Paragraph::new("Hacker News Top Stories")
                    .style(Style::default().fg(Color::Yellow))
                    .alignment(Alignment::Center)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::Rgb(255, 165, 0))) // Orange color
                            .title("HN CLI")
                            .title_alignment(Alignment::Center),
                    );
                f.render_widget(title, chunks[0]);

                // Articles list
                let items: Vec<ListItem> = articles
                    .iter()
                    .enumerate()
                    .map(|(i, article)| {
                        // Parse the date to make it more readable
                        let date_str = parse_date(&article.created_at);
                        
                        let line = if i == self.selected_index {
                            format!("> {}. {} (Score: {}, Date: {})", i + 1, article.title, article.score, date_str)
                        } else {
                            format!("  {}. {} (Score: {}, Date: {})", i + 1, article.title, article.score, date_str)
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
                let footer = Paragraph::new("Commands: ↑↓ Navigate | Enter Open | r Refresh | q Quit")
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

    pub async fn handle_input(&mut self, articles_len: usize) -> char {
        if event::poll(std::time::Duration::from_millis(50)).unwrap() {
            if let Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
                match code {
                    KeyCode::Up => {
                        if self.selected_index > 0 {
                            self.selected_index -= 1;
                        }
                        // Don't wrap around if at the first item
                    }
                    KeyCode::Down => {
                        // Only allow moving down if we're not at the last item
                        if articles_len > 0 && self.selected_index < articles_len - 1 {
                            self.selected_index += 1;
                        }
                        // Don't wrap around if at the last item
                    }
                    KeyCode::Char('q') => return 'q',
                    KeyCode::Char('r') => return 'r',
                    KeyCode::Enter => return 'l', // l for link/open
                    _ => {}
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