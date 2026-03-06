package ui

import (
	"fmt"
	"time"
)

// ParseDate formats an ISO 8601 date string into a readable MM/DD/YYYY format
func ParseDate(dateStr string) string {
	// Try parsing ISO 8601 format (2023-01-01T12:00:00Z)
	if t, err := time.Parse(time.RFC3339, dateStr); err == nil {
		return t.Format("01/02/2006")
	}

	// Fallback: try to extract date part manually
	for i, c := range dateStr {
		if c == 'T' {
			datePart := dateStr[:i]
			return formatDatePart(datePart)
		}
	}

	// If parsing fails, return original string
	return dateStr
}

// formatDatePart converts YYYY-MM-DD to MM/DD/YYYY
func formatDatePart(datePart string) string {
	var year, month, day int
	if _, err := fmt.Sscanf(datePart, "%d-%d-%d", &year, &month, &day); err == nil {
		return fmt.Sprintf("%02d/%02d/%d", month, day, year)
	}
	return datePart
}
