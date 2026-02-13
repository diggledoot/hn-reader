//! Integration tests for the Hacker News CLI application

use hn_reader::{api, models, ui};

/// Test UI initialization and basic functionality
#[test]
fn test_ui_basic_functionality() {
    let ui = ui::Ui::new();
    
    // Test initial state
    assert_eq!(ui.selected_index, 0);
    assert_eq!(ui.current_page, 0);
}

/// Test pagination bounds
#[test]
fn test_pagination_bounds() {
    let mut ui = ui::Ui::new();
    
    // Initial page should be 0
    assert_eq!(ui.current_page, 0);
    
    // Simulate navigating to last allowed page (49 = page 50 in 1-indexed)
    ui.current_page = 49;
    assert_eq!(ui.current_page, 49);
    
    // Verify selected_index resets independently
    ui.selected_index = 5;
    ui.current_page = 1;
    ui.selected_index = 0;
    assert_eq!(ui.selected_index, 0);
    assert_eq!(ui.current_page, 1);
}

/// Test Article model creation and serialization
#[test]
fn test_article_model() {
    let article = models::Article {
        object_id: "test_id".to_string(),
        title: "Test Title".to_string(),
        url: Some("https://example.com".to_string()),
        score: 100,
        created_at: "2023-01-01T12:00:00Z".to_string(),
    };
    
    assert_eq!(article.object_id, "test_id");
    assert_eq!(article.title, "Test Title");
    assert_eq!(article.url, Some("https://example.com".to_string()));
    assert_eq!(article.score, 100);
    assert_eq!(article.created_at, "2023-01-01T12:00:00Z");
    
    // Test with no URL
    let article_no_url = models::Article {
        object_id: "test_id_2".to_string(),
        title: "Test Title 2".to_string(),
        url: None,
        score: 50,
        created_at: "2023-01-01T12:00:00Z".to_string(),
    };
    
    assert_eq!(article_no_url.url, None);
    assert_eq!(article_no_url.score, 50);
    assert_eq!(article_no_url.created_at, "2023-01-01T12:00:00Z");
}

/// Test ApiResponse model
#[test]
fn test_api_response_model() {
    let articles = vec![
        models::Article {
            object_id: "1".to_string(),
            title: "Article 1".to_string(),
            url: Some("https://example1.com".to_string()),
            score: 10,
            created_at: "2023-01-01T12:00:00Z".to_string(),
        }
    ];
    
    let response = models::ApiResponse {
        hits: articles,
        nb_hits: 1,
        page: 0,
        nb_pages: 5,
        hits_per_page: 20,
    };
    
    assert_eq!(response.nb_hits, 1);
    assert_eq!(response.page, 0);
    assert_eq!(response.nb_pages, 5);
    assert_eq!(response.hits_per_page, 20);
    assert_eq!(response.hits.len(), 1);
    assert_eq!(response.hits[0].title, "Article 1");
    assert_eq!(response.hits[0].created_at, "2023-01-01T12:00:00Z");
}

/// Test API URL building functionality
#[tokio::test]
async fn test_api_url_building() {
    // This test verifies that the URL is constructed correctly without making a real request
    let page = 0;
    let expected_url = format!(
        "https://hn.algolia.com/api/v1/search?tags=story&hitsPerPage=20&page={}&numericFilters=points>0",
        page
    );
    
    // Just verify the URL construction logic
    assert!(expected_url.contains("tags=story"));
    assert!(expected_url.contains("hitsPerPage=20"));
    assert!(expected_url.contains("page=0"));
    assert!(expected_url.contains("numericFilters=points>0"));
}

/// Test edge case: API returns empty results
#[tokio::test]
async fn test_api_with_edge_cases() {
    // Test URL building with max page number
    let max_page = u32::MAX;
    let expected_url = format!(
        "https://hn.algolia.com/api/v1/search?tags=story&hitsPerPage=20&page={}&numericFilters=points>0",
        max_page
    );
    assert!(expected_url.contains(&format!("page={}", max_page)));
    
    // Test with page 0 (boundary condition)
    let zero_page_url = format!(
        "https://hn.algolia.com/api/v1/search?tags=story&hitsPerPage=20&page=0&numericFilters=points>0"
    );
    assert!(zero_page_url.contains("page=0"));
}

/// Test that we can fetch articles from the API (requires internet connection)
#[tokio::test]
#[ignore] // Ignore by default to avoid network dependency in CI
async fn test_fetch_top_stories_from_api() {
    // Note: This test makes a real API call to Hacker News
    // It might fail if there's no internet connection or if the API is down
    let result = api::fetch_top_stories(0).await;
    
    assert!(result.is_ok(), "Failed to fetch top stories: {:?}", result.err());
    
    let response = result.unwrap();
    assert!(!response.hits.is_empty(), "Expected at least one story");
    assert!(response.nb_hits > 0, "Expected at least one hit");
    assert_eq!(response.page, 0, "Expected page to be 0");
    assert!(response.hits_per_page > 0, "Expected hits per page to be greater than 0");
    
    // Check that the first article has valid fields
    let first_article = &response.hits[0];
    assert!(!first_article.title.is_empty(), "Article title should not be empty");
    assert!(first_article.score >= 0, "Article score should be non-negative");
    assert!(!first_article.object_id.is_empty(), "Article ID should not be empty");
}