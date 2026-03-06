package ui

import (
	"testing"
)

func TestParseDateValid(t *testing.T) {
	tests := []struct {
		input    string
		expected string
	}{
		{"2023-01-15T12:00:00Z", "01/15/2023"},
		{"2024-12-31T23:59:59Z", "12/31/2024"},
		{"2023-06-15T08:30:00Z", "06/15/2023"},
	}

	for _, test := range tests {
		result := ParseDate(test.input)
		if result != test.expected {
			t.Errorf("ParseDate(%q) = %q, expected %q", test.input, result, test.expected)
		}
	}
}

func TestParseDateInvalid(t *testing.T) {
	tests := []struct {
		input    string
		expected string
	}{
		{"", ""},
		{"invalid-date", "invalid-date"},
		{"2023-01", "2023-01"},
		{"not-a-date", "not-a-date"},
	}

	for _, test := range tests {
		result := ParseDate(test.input)
		if result != test.expected {
			t.Errorf("ParseDate(%q) = %q, expected %q", test.input, result, test.expected)
		}
	}
}

func TestFormatDatePart(t *testing.T) {
	tests := []struct {
		input    string
		expected string
	}{
		{"2023-01-15", "01/15/2023"},
		{"2024-12-31", "12/31/2024"},
		{"2023-06-05", "06/05/2023"},
	}

	for _, test := range tests {
		result := formatDatePart(test.input)
		if result != test.expected {
			t.Errorf("formatDatePart(%q) = %q, expected %q", test.input, result, test.expected)
		}
	}
}
