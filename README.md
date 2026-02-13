```shell
$$                                                             $$\                     
$$ |                                                            $$ |                    
$$$$$$\  $$$$$$$\           $$$$$$\   $$$$$$\   $$$$$$\   $$$$$$$ | $$$$$$\   $$$$$$\  
$$  __$$\ $$  __$$\ $$$$$$\ $$  __$$\ $$  __$$\  \____$$\ $$  __$$ |$$  __$$\ $$  __$$\ 
$$ |  $$ |$$ |  $$ |\______|$$ |  \__|$$$$$$$$ | $$$$$$$ |$$ /  $$ |$$$$$$$$ |$$ |  \__|
$$ |  $$ |$$ |  $$ |        $$ |      $$   ____|$$  __$$ |$$ |  $$ |$$   ____|$$ |      
$$ |  $$ |$$ |  $$ |        $$ |      \$$$$$$$\ \$$$$$$$ |\$$$$$$$ |\$$$$$$$\ $$ |      
\__|  \__|\__|  \__|        \__|       \_______| \_______| \_______| \_______|\__|
```

A terminal-based Hacker News reader.

## Features

- Browse top Hacker News stories from the last 24 hours
- View nested comment threads with proper indentation
- Open articles in your default browser
- Paginated navigation (20 stories per page, up to 50 pages)
- Keyboard-driven TUI with vim-style navigation

## Installation
```bash
git clone <repo>
cd hn-reader
cargo build --release
```

## Usage

| Key | Action |
|-----|--------|
| `↑` / `↓` | Navigate stories |
| `←` / `→` | Previous / Next page |
| `Enter` | Open article in browser |
| `c` | View comments |
| `Esc` / `b` | Back from comments |
| `j` / `k` | Scroll comments |
| `r` | Refresh current page |
| `q` | Quit |
