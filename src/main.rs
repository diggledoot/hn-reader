use hn_reader::{api::fetch_top_stories, models, ui::Ui};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ui = Ui::new();
    ui.setup_terminal()?;

    let mut should_refresh = true;
    let mut articles: Option<Vec<models::Article>> = None;
    let mut total_pages: u32 = 1; // Will be updated from API response

    loop {
        if should_refresh {
            // Show loading indicator
            ui.render_loading();

            // Fetch articles for the current page
            match fetch_top_stories(ui.current_page).await {
                Ok(mut response) => {
                    // Cap total pages at 50
                    total_pages = response.nb_pages.min(50);
                    // Sort articles by score in descending order
                    response.hits.sort_by(|a, b| b.score.cmp(&a.score));
                    articles = Some(response.hits);

                    // Render the articles
                    if let Some(ref arts) = articles {
                        ui.render(arts, total_pages);
                    }
                }
                Err(e) => {
                    eprintln!("Error fetching articles: {}", e);
                    break;
                }
            }
            should_refresh = false;
        }

        // Handle user input
        match ui.handle_input(articles.as_ref().map(Vec::len).unwrap_or(0), total_pages).await {
            'q' => break, // Quit the application
            'r' => {
                // Refresh current page
                should_refresh = true;
            },
            'n' | 'p' => {
                // Next or previous page (current_page already updated by handle_input)
                should_refresh = true;
            },
            'l' => {
                // Open selected article
                if let Some(ref arts) = articles {
                    ui.open_selected_article(arts);
                    // Re-render after opening link to maintain UI state
                    ui.render(arts, total_pages);
                }
            },
            _ => {
                // Other inputs, just refresh the display to update cursor position
                if let Some(ref arts) = articles {
                    ui.render(arts, total_pages);
                }
            }
        }
    }

    ui.cleanup_terminal();
    Ok(())
}
