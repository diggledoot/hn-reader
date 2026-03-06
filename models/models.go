package models

// Article represents a Hacker News story from the Algolia API
type Article struct {
	ObjectID  string  `json:"objectID"`
	Title     string  `json:"title"`
	URL       *string `json:"url,omitempty"`
	Score     int     `json:"points"`
	CreatedAt string  `json:"created_at"`
}

// ApiResponse represents the response from the Algolia search API
type ApiResponse struct {
	Hits          []Article `json:"hits"`
	TotalHits     int       `json:"nbHits"`
	Page          int       `json:"page"`
	TotalPages    int       `json:"nbPages"`
	HitsPerPage   int       `json:"hitsPerPage"`
}

// Comment represents a comment on a Hacker News story
type Comment struct {
	ID        uint64    `json:"id"`
	Author    *string   `json:"author,omitempty"`
	Text      *string   `json:"text,omitempty"`
	CreatedAt *string   `json:"created_at,omitempty"`
	Children  []Comment `json:"children"`
}

// ItemResponse represents the response for a single item (story/comments)
type ItemResponse struct {
	ID       uint64    `json:"id"`
	Title    *string   `json:"title,omitempty"`
	Author   *string   `json:"author,omitempty"`
	Type     *string   `json:"type,omitempty"`
	Children []Comment `json:"children"`
}
