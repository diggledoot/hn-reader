```shell
$$                                                             $$\
$$ |                                                            $$ |
$$$$$$\  $$$$$$$\           $$$$$$\   $$$$$$\   $$$$$$\   $$$$$$$ | $$$$$$\   $$$$$$\
$$  __$$\ $$  __$$\ $$$$$$\ $$  __$$\ $$  __$$\  \____$$\ $$  __$$ |$$  __$$\ $$  __$$\
$$ |  $$ |$$ |  $$ | $$$$$$\ $$ |  \__|$$$$$$$$ | $$$$$$$ |$$ /  $$ |$$$$$$$$ |$$ |  \__|
$$ |  $$ |$$ |  $$ |\______|$$ |      $$   ____|$$  __$$ |$$ |  $$ |$$   ____|$$ |
$$ |  $$ |$$ |  $$ |        $$ |      \$$$$$$$\ \$$$$$$$ |\$$$$$$$ |\$$$$$$$\ $$ |
\__|  \__|\__|  \__|        \__|       \_______| \_______| \_______| \_______|\__|
```

A terminal-based Hacker News reader built with Go and Bubble Tea.

## Features

- Browse top Hacker News stories from the last 24 hours
- Open articles in your default browser
- Paginated navigation (20 stories per page, up to 50 pages)
- Keyboard-driven TUI with vim-style navigation
- Sorted by score (highest first)

## Requirements

- Go 1.21 or later

## Installation

```bash
git clone <repo>
cd hn-reader
go mod tidy
go build -o hn-reader
```

## Usage

Run the application:

```bash
./hn-reader
```

## Key Bindings

| Key | Action |
|-----|--------|
| `↑` / `k` | Navigate up |
| `↓` / `j` | Navigate down |
| `←` / `h` | Previous page |
| `→` / `l` | Next page |
| `Enter` | Open article in browser |
| `c` | Open HN discussion thread |
| `r` | Refresh current page |
| `q` | Quit |

## Development

Run tests:

```bash
go test ./...
```

Run with verbose output:

```bash
go test -v ./...
```

## Tech Stack

- **Bubble Tea** - Functional terminal UI framework
- **Lip Gloss** - Styling and layout for terminal UIs
- **open-golang** - Open URLs in default browser

## License

MIT
