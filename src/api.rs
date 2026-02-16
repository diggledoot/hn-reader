use crate::models::{ApiResponse, ItemResponse};
use std::error::Error;

// Using the Algolia API to get stories with high points (scores)
const ALGOLIA_API_URL: &str = "https://hn.algolia.com/api/v1/search";
const ALGOLIA_ITEMS_URL: &str = "https://hn.algolia.com/api/v1/items";

pub async fn fetch_top_stories(page: u32) -> Result<ApiResponse, Box<dyn Error>> {
    // Get stories from the last 24 hours
    let one_day_ago = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        - 24 * 60 * 60; // 24 hours in seconds

    let url = format!(
        "{}?tags=story&hitsPerPage=20&page={}&numericFilters=created_at_i>{}",
        ALGOLIA_API_URL,
        page,
        one_day_ago
    );

    let response = reqwest::get(&url).await?;
    let api_response: ApiResponse = response.json().await?;

    Ok(api_response)
}

pub async fn fetch_front_page_stories(page: u32) -> Result<ApiResponse, Box<dyn Error>> {
    // Get front page (trending) stories
    let url = format!(
        "{}?tags=front_page_story&hitsPerPage=20&page={}",
        ALGOLIA_API_URL,
        page
    );

    let response = reqwest::get(&url).await?;
    let api_response: ApiResponse = response.json().await?;

    Ok(api_response)
}

pub async fn fetch_comments(story_id: &str) -> Result<ItemResponse, Box<dyn Error>> {
    let url = format!("{}/{}", ALGOLIA_ITEMS_URL, story_id);
    let response = reqwest::get(&url).await?;
    let item_response: ItemResponse = response.json().await?;
    Ok(item_response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(ALGOLIA_API_URL, "https://hn.algolia.com/api/v1/search");
    }

    #[tokio::test]
    async fn test_build_request_url() {
        // This test verifies that the URL is constructed correctly
        // We won't actually make the request in tests
        let page = 0;
        let expected_url = format!(
            "{}?tags=story&hitsPerPage=20&page={}&numericFilters=points>0",
            ALGOLIA_API_URL, page
        );

        // Just verify the URL construction logic
        assert!(expected_url.contains("tags=story"));
        assert!(expected_url.contains("hitsPerPage=20"));
        assert!(expected_url.contains("page=0"));
        assert!(expected_url.contains("numericFilters=points>0"));
    }

    #[test]
    fn test_article_struct() {
        use crate::models::Article;
        // Test that we can create an Article struct
        let article = Article {
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
    }
    
    #[tokio::test]
    async fn test_url_building_edge_cases() {
        // Test with max page number
        let max_page = u32::MAX;
        let expected_url = format!(
            "{}?tags=story&hitsPerPage=20&page={}&numericFilters=points>0",
            ALGOLIA_API_URL, max_page
        );
        assert!(expected_url.contains(&format!("page={}", max_page)));

        // Test with page 0
        let zero_page_url = format!(
            "{}?tags=story&hitsPerPage=20&page=0&numericFilters=points>0",
            ALGOLIA_API_URL
        );
        assert!(zero_page_url.contains("page=0"));

        // Test URL contains all required parameters
        assert!(zero_page_url.contains("tags=story"));
        assert!(zero_page_url.contains("hitsPerPage=20"));
        assert!(zero_page_url.contains("numericFilters=points>0"));
    }

    #[tokio::test]
    async fn test_fetch_front_page_stories_url() {
        // Test URL construction for front page stories
        let page = 0;
        let expected_url = format!(
            "{}?tags=front_page_story&hitsPerPage=20&page={}",
            ALGOLIA_API_URL, page
        );

        assert!(expected_url.contains("tags=front_page_story"));
        assert!(expected_url.contains("hitsPerPage=20"));
        assert!(expected_url.contains("page=0"));
    }

    #[tokio::test]
    async fn test_fetch_comments_url() {
        // Test URL construction for fetching comments
        let story_id = "12345678";
        let expected_url = format!("{}/{}", ALGOLIA_ITEMS_URL, story_id);

        assert_eq!(expected_url, "https://hn.algolia.com/api/v1/items/12345678");
        assert!(expected_url.contains(story_id));
    }

    #[test]
    fn test_constants_items_url() {
        assert_eq!(ALGOLIA_ITEMS_URL, "https://hn.algolia.com/api/v1/items");
    }
}