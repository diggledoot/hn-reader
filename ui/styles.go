package ui

import (
	"github.com/charmbracelet/lipgloss"
)

// Colors
var (
	// Orange color for HN branding
	OrangeColor = lipgloss.Color("#FFA500")
	YellowColor = lipgloss.Color("#FFFF00")
	BlueColor   = lipgloss.Color("#0000FF")
	GrayColor   = lipgloss.Color("#808080")
)

// BaseStyle is the default block style with orange borders
var BaseStyle = lipgloss.NewStyle().
	Border(lipgloss.RoundedBorder()).
	BorderForeground(OrangeColor)

// TitleStyle for the header
var TitleStyle = lipgloss.NewStyle().
	Foreground(YellowColor).
	Bold(true)

// FooterStyle for the command hints
var FooterStyle = lipgloss.NewStyle().
	Foreground(lipgloss.Color("#666666"))

// HighlightStyle for selected items
var HighlightStyle = lipgloss.NewStyle().
	Background(BlueColor).
	Foreground(lipgloss.Color("#FFFFFF")).
	Bold(true)

// LoadingStyle for loading indicator
var LoadingStyle = lipgloss.NewStyle().
	Foreground(YellowColor)

// ErrorStyle for error messages
var ErrorStyle = lipgloss.NewStyle().
	Foreground(lipgloss.Color("#FF0000"))

// AppTitle is the main application title
var AppTitle = lipgloss.NewStyle().
	Border(lipgloss.RoundedBorder()).
	BorderForeground(OrangeColor).
	Foreground(YellowColor).
	Bold(true).
	Padding(0, 1)
