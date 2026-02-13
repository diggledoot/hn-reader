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

#[derive(Deserialize, Debug, Clone)]
pub struct Comment {
    pub id: u64,
    pub author: Option<String>,
    pub text: Option<String>,
    pub created_at: Option<String>,
    #[serde(default)]
    pub children: Vec<Comment>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ItemResponse {
    pub id: u64,
    pub title: Option<String>,
    pub author: Option<String>,
    #[serde(default)]
    pub children: Vec<Comment>,
    #[serde(rename = "type")]
    pub item_type: Option<String>,
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
        let articles = vec![Article {
            object_id: "1".to_string(),
            title: "Article 1".to_string(),
            url: Some("https://example1.com".to_string()),
            score: 10,
            created_at: "2023-01-01T12:00:00Z".to_string(),
        }];

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
    fn test_comment_deserialization() {
        let json = r#"{
            "id": 123,
            "author": "testuser",
            "text": "Hello <p>World",
            "created_at": "2025-01-01T12:00:00.000Z",
            "children": []
        }"#;
        let comment: Comment = serde_json::from_str(json).unwrap();
        assert_eq!(comment.id, 123);
        assert_eq!(comment.author, Some("testuser".to_string()));
        assert_eq!(comment.text, Some("Hello <p>World".to_string()));
        assert!(comment.children.is_empty());
    }

    #[test]
    fn test_comment_with_nested_children() {
        let json = r#"{
            "id": 1,
            "author": "alice",
            "text": "parent",
            "created_at": "2025-01-01T12:00:00.000Z",
            "children": [{
                "id": 2,
                "author": "bob",
                "text": "child",
                "created_at": "2025-01-01T13:00:00.000Z",
                "children": []
            }]
        }"#;
        let comment: Comment = serde_json::from_str(json).unwrap();
        assert_eq!(comment.children.len(), 1);
        assert_eq!(comment.children[0].author, Some("bob".to_string()));
    }

    #[test]
    fn test_item_response_deserialization() {
        let json = r#"{
            "id": 999,
            "title": "Test Story",
            "author": "poster",
            "type": "story",
            "children": [{
                "id": 1000,
                "author": "commenter",
                "text": "nice post",
                "created_at": "2025-01-01T12:00:00.000Z",
                "children": []
            }]
        }"#;
        let item: ItemResponse = serde_json::from_str(json).unwrap();
        assert_eq!(item.id, 999);
        assert_eq!(item.title, Some("Test Story".to_string()));
        assert_eq!(item.item_type, Some("story".to_string()));
        assert_eq!(item.children.len(), 1);
    }

    #[test]
    fn test_comment_with_null_fields() {
        let json = r#"{
            "id": 456,
            "author": null,
            "text": null,
            "created_at": null,
            "children": []
        }"#;
        let comment: Comment = serde_json::from_str(json).unwrap();
        assert_eq!(comment.id, 456);
        assert_eq!(comment.author, None);
        assert_eq!(comment.text, None);
        assert_eq!(comment.created_at, None);
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
