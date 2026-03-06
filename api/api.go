package api

import (
	"encoding/json"
	"fmt"
	"net/http"
	"net/url"
	"time"

	"hn-reader/models"
)

const (
	// Algolia API endpoints for Hacker News
	AlgoliaAPIURL   = "https://hn.algolia.com/api/v1/search"
	AlgoliaItemsURL = "https://hn.algolia.com/api/v1/items"
)

// HTTPClient is the default HTTP client with a 30 second timeout
var HTTPClient = &http.Client{
	Timeout: 30 * time.Second,
}

// FetchTopStories fetches stories from the last 24 hours with points > 0
func FetchTopStories(page int) (*models.ApiResponse, error) {
	// Calculate timestamp for 24 hours ago
	oneDayAgo := time.Now().Add(-24 * time.Hour).Unix()

	// Build URL with proper encoding using url.Values
	params := url.Values{}
	params.Set("tags", "story")
	params.Set("hitsPerPage", "20")
	params.Set("page", fmt.Sprintf("%d", page))
	params.Set("numericFilters", fmt.Sprintf("created_at_i>%d", oneDayAgo))

	encodedURL := fmt.Sprintf("%s?%s", AlgoliaAPIURL, params.Encode())

	resp, err := HTTPClient.Get(encodedURL)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch stories: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("API returned status %d", resp.StatusCode)
	}

	var apiResponse models.ApiResponse
	if err := json.NewDecoder(resp.Body).Decode(&apiResponse); err != nil {
		return nil, fmt.Errorf("failed to decode response: %w", err)
	}

	return &apiResponse, nil
}

// FetchFrontPageStories fetches trending/front page stories
func FetchFrontPageStories(page int) (*models.ApiResponse, error) {
	url := fmt.Sprintf(
		"%s?tags=front_page_story&hitsPerPage=20&page=%d",
		AlgoliaAPIURL,
		page,
	)

	resp, err := HTTPClient.Get(url)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch front page stories: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("API returned status %d", resp.StatusCode)
	}

	var apiResponse models.ApiResponse
	if err := json.NewDecoder(resp.Body).Decode(&apiResponse); err != nil {
		return nil, fmt.Errorf("failed to decode response: %w", err)
	}

	return &apiResponse, nil
}

// FetchComments fetches comments for a specific story
func FetchComments(storyID string) (*models.ItemResponse, error) {
	url := fmt.Sprintf("%s/%s", AlgoliaItemsURL, storyID)

	resp, err := HTTPClient.Get(url)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch comments: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("API returned status %d", resp.StatusCode)
	}

	var itemResponse models.ItemResponse
	if err := json.NewDecoder(resp.Body).Decode(&itemResponse); err != nil {
		return nil, fmt.Errorf("failed to decode response: %w", err)
	}

	return &itemResponse, nil
}
