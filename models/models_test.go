package models

import (
	"encoding/json"
	"testing"
)

func TestArticleCreation(t *testing.T) {
	article := Article{
		ObjectID:  "123",
		Title:     "Test Article",
		URL:       strPtr("https://example.com"),
		Score:     100,
		CreatedAt: "2023-01-01T12:00:00Z",
	}

	if article.ObjectID != "123" {
		t.Errorf("Expected ObjectID to be 123, got %s", article.ObjectID)
	}
	if article.Title != "Test Article" {
		t.Errorf("Expected Title to be 'Test Article', got %s", article.Title)
	}
	if *article.URL != "https://example.com" {
		t.Errorf("Expected URL to be 'https://example.com', got %s", *article.URL)
	}
	if article.Score != 100 {
		t.Errorf("Expected Score to be 100, got %d", article.Score)
	}
}

func TestArticleWithoutURL(t *testing.T) {
	article := Article{
		ObjectID:  "456",
		Title:     "Test Article Without URL",
		URL:       nil,
		Score:     50,
		CreatedAt: "2023-01-01T12:00:00Z",
	}

	if article.URL != nil {
		t.Errorf("Expected URL to be nil")
	}
}

func TestApiResponseCreation(t *testing.T) {
	articles := []Article{
		{
			ObjectID:  "1",
			Title:     "Article 1",
			URL:       strPtr("https://example1.com"),
			Score:     10,
			CreatedAt: "2023-01-01T12:00:00Z",
		},
	}

	response := ApiResponse{
		Hits:        articles,
		TotalHits:   1,
		Page:        0,
		TotalPages:  5,
		HitsPerPage: 20,
	}

	if response.TotalHits != 1 {
		t.Errorf("Expected TotalHits to be 1, got %d", response.TotalHits)
	}
	if response.Page != 0 {
		t.Errorf("Expected Page to be 0, got %d", response.Page)
	}
	if response.TotalPages != 5 {
		t.Errorf("Expected TotalPages to be 5, got %d", response.TotalPages)
	}
	if response.HitsPerPage != 20 {
		t.Errorf("Expected HitsPerPage to be 20, got %d", response.HitsPerPage)
	}
}

func TestArticleJSONDeserialization(t *testing.T) {
	jsonStr := `{
		"objectID": "test123",
		"title": "Test Story",
		"url": "https://example.com",
		"points": 150,
		"created_at": "2023-01-15T10:30:00Z"
	}`

	var article Article
	if err := json.Unmarshal([]byte(jsonStr), &article); err != nil {
		t.Fatalf("Failed to unmarshal article: %v", err)
	}

	if article.ObjectID != "test123" {
		t.Errorf("Expected ObjectID to be 'test123', got %s", article.ObjectID)
	}
	if article.Title != "Test Story" {
		t.Errorf("Expected Title to be 'Test Story', got %s", article.Title)
	}
	if *article.URL != "https://example.com" {
		t.Errorf("Expected URL to be 'https://example.com', got %s", *article.URL)
	}
	if article.Score != 150 {
		t.Errorf("Expected Score to be 150, got %d", article.Score)
	}
}

func TestCommentDeserialization(t *testing.T) {
	jsonStr := `{
		"id": 123,
		"author": "testuser",
		"text": "Hello World",
		"created_at": "2025-01-01T12:00:00.000Z",
		"children": []
	}`

	var comment Comment
	if err := json.Unmarshal([]byte(jsonStr), &comment); err != nil {
		t.Fatalf("Failed to unmarshal comment: %v", err)
	}

	if comment.ID != 123 {
		t.Errorf("Expected ID to be 123, got %d", comment.ID)
	}
	if comment.Author == nil || *comment.Author != "testuser" {
		t.Errorf("Expected Author to be 'testuser'")
	}
	if comment.Text == nil || *comment.Text != "Hello World" {
		t.Errorf("Expected Text to be 'Hello World'")
	}
}

func TestCommentWithNestedChildren(t *testing.T) {
	jsonStr := `{
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
	}`

	var comment Comment
	if err := json.Unmarshal([]byte(jsonStr), &comment); err != nil {
		t.Fatalf("Failed to unmarshal comment: %v", err)
	}

	if len(comment.Children) != 1 {
		t.Errorf("Expected 1 child, got %d", len(comment.Children))
	}
	if comment.Children[0].Author == nil || *comment.Children[0].Author != "bob" {
		t.Errorf("Expected child author to be 'bob'")
	}
}

func TestItemResponseDeserialization(t *testing.T) {
	jsonStr := `{
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
	}`

	var item ItemResponse
	if err := json.Unmarshal([]byte(jsonStr), &item); err != nil {
		t.Fatalf("Failed to unmarshal item: %v", err)
	}

	if item.ID != 999 {
		t.Errorf("Expected ID to be 999, got %d", item.ID)
	}
	if item.Title == nil || *item.Title != "Test Story" {
		t.Errorf("Expected Title to be 'Test Story'")
	}
	if item.Type == nil || *item.Type != "story" {
		t.Errorf("Expected Type to be 'story'")
	}
	if len(item.Children) != 1 {
		t.Errorf("Expected 1 child, got %d", len(item.Children))
	}
}

func TestCommentWithNullFields(t *testing.T) {
	jsonStr := `{
		"id": 456,
		"author": null,
		"text": null,
		"created_at": null,
		"children": []
	}`

	var comment Comment
	if err := json.Unmarshal([]byte(jsonStr), &comment); err != nil {
		t.Fatalf("Failed to unmarshal comment: %v", err)
	}

	if comment.ID != 456 {
		t.Errorf("Expected ID to be 456, got %d", comment.ID)
	}
	if comment.Author != nil {
		t.Errorf("Expected Author to be nil")
	}
	if comment.Text != nil {
		t.Errorf("Expected Text to be nil")
	}
}

func TestApiResponseEdgeCases(t *testing.T) {
	// Empty response
	emptyResponse := ApiResponse{
		Hits:        []Article{},
		TotalHits:   0,
		Page:        0,
		TotalPages:  0,
		HitsPerPage: 0,
	}

	if len(emptyResponse.Hits) != 0 {
		t.Errorf("Expected empty Hits")
	}
	if emptyResponse.TotalHits != 0 {
		t.Errorf("Expected TotalHits to be 0")
	}
}

func strPtr(s string) *string {
	return &s
}
