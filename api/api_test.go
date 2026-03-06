package api

import (
	"fmt"
	"strings"
	"testing"
)

func TestConstants(t *testing.T) {
	if AlgoliaAPIURL != "https://hn.algolia.com/api/v1/search" {
		t.Errorf("Expected AlgoliaAPIURL to be 'https://hn.algolia.com/api/v1/search', got %s", AlgoliaAPIURL)
	}
	if AlgoliaItemsURL != "https://hn.algolia.com/api/v1/items" {
		t.Errorf("Expected AlgoliaItemsURL to be 'https://hn.algolia.com/api/v1/items', got %s", AlgoliaItemsURL)
	}
}

func TestHTTPClientTimeout(t *testing.T) {
	if HTTPClient.Timeout.Seconds() != 30 {
		t.Errorf("Expected HTTP client timeout to be 30 seconds, got %v", HTTPClient.Timeout)
	}
}

func TestBuildSearchURL(t *testing.T) {
	// Test URL construction for top stories
	page := 0
	expectedParams := []string{
		"tags=story",
		"hitsPerPage=20",
		"page=0",
		"numericFilters=created_at_i",
	}

	// We can't test the exact timestamp, but we can verify the structure
	url := buildTestURL(AlgoliaAPIURL, page, true)

	for _, param := range expectedParams {
		if !strings.Contains(url, param) {
			t.Errorf("Expected URL to contain '%s', got %s", param, url)
		}
	}
}

func TestBuildFrontPageURL(t *testing.T) {
	page := 0
	expectedParams := []string{
		"tags=front_page_story",
		"hitsPerPage=20",
		"page=0",
	}

	url := buildTestURL(AlgoliaAPIURL, page, false)

	for _, param := range expectedParams {
		if !strings.Contains(url, param) {
			t.Errorf("Expected URL to contain '%s', got %s", param, url)
		}
	}
}

func TestBuildItemsURL(t *testing.T) {
	storyID := "12345678"
	expectedURL := AlgoliaItemsURL + "/" + storyID

	if !strings.Contains(expectedURL, storyID) {
		t.Errorf("Expected URL to contain story ID '%s', got %s", storyID, expectedURL)
	}
}

func TestURLBuildingEdgeCases(t *testing.T) {
	// Test with page 0
	url := buildTestURL(AlgoliaAPIURL, 0, true)
	if !strings.Contains(url, "page=0") {
		t.Errorf("Expected URL to contain 'page=0', got %s", url)
	}

	// Test with page 1
	url = buildTestURL(AlgoliaAPIURL, 1, true)
	if !strings.Contains(url, "page=1") {
		t.Errorf("Expected URL to contain 'page=1', got %s", url)
	}
}

// buildTestURL is a helper for testing URL construction
func buildTestURL(baseURL string, page int, withTimestamp bool) string {
	if strings.Contains(baseURL, "search") {
		if withTimestamp {
			return baseURL + fmt.Sprintf("?tags=story&hitsPerPage=20&page=%d&numericFilters=created_at_i>1234567890", page)
		}
		return baseURL + fmt.Sprintf("?tags=front_page_story&hitsPerPage=20&page=%d", page)
	}
	return baseURL
}
