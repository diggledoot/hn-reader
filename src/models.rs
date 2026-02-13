use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Article {
    #[serde(rename = "objectID")]
    pub object_id: String,
    pub title: String,
    pub url: Option<String>,
    #[serde(rename = "points")]
    pub score: i32,
    #[serde(rename = "created_at")]
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiResponse {
    pub hits: Vec<Article>,
    #[serde(rename = "nbHits")]
    pub nb_hits: u32,
    pub page: u32,
    #[serde(rename = "nbPages")]
    pub nb_pages: u32,
    #[serde(rename = "hitsPerPage")]
    pub hits_per_page: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article_creation() {
        let article = Article {
            object_id: "123".to_string(),
            title: "Test Article".to_string(),
            url: Some("https://example.com".to_string()),
            score: 100,
            created_at: "2023-01-01T12:00:00Z".to_string(),
        };

        assert_eq!(article.object_id, "123");
        assert_eq!(article.title, "Test Article");
        assert_eq!(article.url, Some("https://example.com".to_string()));
        assert_eq!(article.score, 100);
        assert_eq!(article.created_at, "2023-01-01T12:00:00Z");
    }

    #[test]
    fn test_article_without_url() {
        let article = Article {
            object_id: "456".to_string(),
            title: "Test Article Without URL".to_string(),
            url: None,
            score: 50,
            created_at: "2023-01-01T12:00:00Z".to_string(),
        };

        assert_eq!(article.object_id, "456");
        assert_eq!(article.title, "Test Article Without URL");
        assert_eq!(article.url, None);
        assert_eq!(article.score, 50);
        assert_eq!(article.created_at, "2023-01-01T12:00:00Z");
    }

    #[test]
    fn test_api_response_creation() {
        let articles = vec![
            Article {
                object_id: "1".to_string(),
                title: "Article 1".to_string(),
                url: Some("https://example1.com".to_string()),
                score: 10,
                created_at: "2023-01-01T12:00:00Z".to_string(),
            }
        ];
        
        let response = ApiResponse {
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
    }
    
    #[test]
    fn test_article_with_empty_fields() {
        // Edge case: Article with empty title
        let article = Article {
            object_id: "empty_title".to_string(),
            title: "".to_string(),
            url: None,
            score: 0,
            created_at: "2023-01-01T12:00:00Z".to_string(),
        };
        
        assert_eq!(article.object_id, "empty_title");
        assert_eq!(article.title, "");
        assert_eq!(article.url, None);
        assert_eq!(article.score, 0);
        assert_eq!(article.created_at, "2023-01-01T12:00:00Z");
    }
    
    #[test]
    fn test_api_response_edge_cases() {
        // Edge case: Empty response
        let empty_response = ApiResponse {
            hits: vec![],
            nb_hits: 0,
            page: 0,
            nb_pages: 0,
            hits_per_page: 0,
        };
        
        assert_eq!(empty_response.hits.len(), 0);
        assert_eq!(empty_response.nb_hits, 0);
        assert_eq!(empty_response.nb_pages, 0);
        assert_eq!(empty_response.hits_per_page, 0);
        
        // Edge case: Max values
        let max_response = ApiResponse {
            hits: vec![],
            nb_hits: u32::MAX,
            page: u32::MAX,
            nb_pages: u32::MAX,
            hits_per_page: u32::MAX,
        };
        
        assert_eq!(max_response.nb_hits, u32::MAX);
        assert_eq!(max_response.page, u32::MAX);
        assert_eq!(max_response.nb_pages, u32::MAX);
        assert_eq!(max_response.hits_per_page, u32::MAX);
    }
}