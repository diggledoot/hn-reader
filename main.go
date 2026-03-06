package main

import (
	"fmt"
	"os"
	"sort"
	"strings"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
	"github.com/skratchdot/open-golang/open"

	"hn-reader/api"
	"hn-reader/models"
	"hn-reader/ui"
)

// MaxPages caps the total pages at 50
const MaxPages = 50

// ViewMode represents the current view state
type ViewMode string

const (
	ViewModeArticles ViewMode = "articles"
)

// fetchResultMsg is sent when API fetch completes
type fetchResultMsg struct {
	response   *models.ApiResponse
	err        error
	totalPages int
}

// Model represents the application state
type Model struct {
	articles      []models.Article
	selectedIndex int
	currentPage   int
	totalPages    int
	loading       bool
	err           error
	viewMode      ViewMode
	width         int
	height        int
}

// NewModel creates a new model with default values
func NewModel() Model {
	return Model{
		selectedIndex: 0,
		currentPage:   0,
		totalPages:    1,
		loading:       true,
		viewMode:      ViewModeArticles,
	}
}

// Init initializes the application
func (m Model) Init() tea.Cmd {
	return fetchStories(m.currentPage)
}

// fetchStories returns a command that fetches stories
func fetchStories(page int) tea.Cmd {
	return func() tea.Msg {
		response, err := api.FetchTopStories(page)
		if err != nil {
			return fetchResultMsg{err: err}
		}

		// Cap total pages at MaxPages
		totalPages := response.TotalPages
		if totalPages > MaxPages {
			totalPages = MaxPages
		}

		// Sort articles by score in descending order
		sort.Slice(response.Hits, func(i, j int) bool {
			return response.Hits[i].Score > response.Hits[j].Score
		})

		return fetchResultMsg{
			response:   response,
			totalPages: totalPages,
		}
	}
}

// Update handles messages and updates the model
func (m Model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		return m.handleKeyPress(msg)

	case fetchResultMsg:
		if msg.err != nil {
			m.err = msg.err
			m.loading = false
			return m, nil
		}

		m.articles = msg.response.Hits
		m.totalPages = msg.totalPages
		m.loading = false
		m.selectedIndex = 0
		return m, nil

	case tea.WindowSizeMsg:
		m.width = msg.Width
		m.height = msg.Height
		return m, nil
	}

	return m, nil
}

// handleKeyPress processes keyboard input
func (m Model) handleKeyPress(msg tea.KeyMsg) (tea.Model, tea.Cmd) {
	switch msg.String() {
	case "q", "ctrl+c":
		return m, tea.Quit

	case "r":
		// Refresh current page
		m.loading = true
		return m, fetchStories(m.currentPage)

	case "up", "k":
		if m.selectedIndex > 0 {
			m.selectedIndex--
		}
		return m, nil

	case "down", "j":
		if m.selectedIndex < len(m.articles)-1 {
			m.selectedIndex++
		}
		return m, nil

	case "left", "h":
		if m.currentPage > 0 {
			m.currentPage--
			m.selectedIndex = 0
			m.loading = true
			return m, fetchStories(m.currentPage)
		}
		return m, nil

	case "right", "l":
		if m.currentPage < m.totalPages-1 {
			m.currentPage++
			m.selectedIndex = 0
			m.loading = true
			return m, fetchStories(m.currentPage)
		}
		return m, nil

	case "enter":
		m.openSelectedArticle()
		return m, nil

	case "c":
		m.openHNDiscussion()
		return m, nil
	}

	return m, nil
}

// openSelectedArticle opens the selected article URL in the browser
func (m Model) openSelectedArticle() {
	if m.selectedIndex >= len(m.articles) {
		return
	}

	article := m.articles[m.selectedIndex]
	if article.URL != nil && *article.URL != "" {
		if err := open.Run(*article.URL); err != nil {
			// Silently fail - user can see the URL in the list
		}
	}
}

// openHNDiscussion opens the HN discussion thread
func (m Model) openHNDiscussion() {
	if m.selectedIndex >= len(m.articles) {
		return
	}

	article := m.articles[m.selectedIndex]
	hnURL := fmt.Sprintf("https://news.ycombinator.com/item?id=%s", article.ObjectID)
	if err := open.Run(hnURL); err != nil {
		// Silently fail
	}
}

// View renders the UI
func (m Model) View() string {
	var b strings.Builder

	// Header
	header := m.renderHeader()
	b.WriteString(header)
	b.WriteString("\n")

	// Content
	if m.loading {
		b.WriteString(m.renderLoading())
	} else if m.err != nil {
		b.WriteString(m.renderError())
	} else {
		b.WriteString(m.renderArticles())
	}

	b.WriteString("\n")

	// Footer
	footer := m.renderFooter()
	b.WriteString(footer)

	return b.String()
}

// renderHeader renders the application header
func (m Model) renderHeader() string {
	title := fmt.Sprintf("Hacker News Top Stories — Page %d / %d", m.currentPage+1, m.totalPages)
	return ui.AppTitle.Width(m.width - 2).Render(title)
}

// renderLoading renders the loading indicator
func (m Model) renderLoading() string {
	loading := fmt.Sprintf("Loading page %d...", m.currentPage+1)
	style := ui.BaseStyle.
		Width(m.width - 2).
		Align(lipgloss.Center)
	return style.Render(ui.LoadingStyle.Render(loading))
}

// renderError renders the error message
func (m Model) renderError() string {
	style := ui.BaseStyle.
		Width(m.width - 2)
	return style.Render(ui.ErrorStyle.Render(fmt.Sprintf("Error: %v", m.err)))
}

// renderArticles renders the list of articles
func (m Model) renderArticles() string {
	var items []string

	pageOffset := m.currentPage * 20
	for i, article := range m.articles {
		dateStr := ui.ParseDate(article.CreatedAt)
		globalIndex := pageOffset + i + 1

		line := fmt.Sprintf("  %d. %s (Score: %d, Date: %s)",
			globalIndex, article.Title, article.Score, dateStr)

		if i == m.selectedIndex {
			line = fmt.Sprintf("> %d. %s (Score: %d, Date: %s)",
				globalIndex, article.Title, article.Score, dateStr)
			items = append(items, ui.HighlightStyle.Render(line))
		} else {
			items = append(items, line)
		}
	}

	content := strings.Join(items, "\n")
	style := ui.BaseStyle.Width(m.width - 2)
	return style.Render(content)
}

// renderFooter renders the footer with command hints
func (m Model) renderFooter() string {
	commands := "Commands: ↑↓/jk Navigate | ←→/hl Page | Enter Open | c Comments | r Refresh | q Quit"
	style := ui.BaseStyle.
		Width(m.width - 2).
		Align(lipgloss.Center)
	return style.Render(ui.FooterStyle.Render(commands))
}

func main() {
	p := tea.NewProgram(NewModel(), tea.WithAltScreen())
	if _, err := p.Run(); err != nil {
		fmt.Fprintf(os.Stderr, "Error running program: %v\n", err)
		os.Exit(1)
	}
}
