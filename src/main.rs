use hn_cli::{api::fetch_top_stories, models, ui::Ui};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ui = Ui::new();
    ui.setup_terminal()?;

    let mut should_refresh = true;
    let mut articles: Option<Vec<models::Article>> = None;

    loop {
        if should_refresh {
            // Fetch the top 20 articles (page 0)
            match fetch_top_stories(0).await {
                Ok(mut response) => {
                    // Sort articles by score in descending order
                    response.hits.sort_by(|a, b| b.score.cmp(&a.score));
                    articles = Some(response.hits);

                    // Render the articles
                    if let Some(ref arts) = articles {
                        ui.render(arts);
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
        match ui.handle_input(articles.as_ref().map(Vec::len).unwrap_or(0)).await {
            'q' => break, // Quit the application
            'r' => {
                // Refresh current page
                should_refresh = true;
            },
            'l' => {
                // Open selected article
                if let Some(ref arts) = articles {
                    ui.open_selected_article(arts);
                    // Re-render after opening link to maintain UI state
                    ui.render(arts);
                }
            },
            _ => {
                // Other inputs, just refresh the display to update cursor position
                if let Some(ref arts) = articles {
                    ui.render(arts);
                }
            }
        }
    }

    ui.cleanup_terminal();
    Ok(())
}
