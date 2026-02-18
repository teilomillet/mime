# Mime

A distraction-free note-taking app built with Rust and Dioxus.

## Features

- **Markdown Editor** with live preview
- **Quick Switcher** (Ctrl+K) - fuzzy search to jump between notes
- **Focus Mode** (Ctrl+Shift+F) - hide all UI, just you and your writing
- **Document Outline** - click headings to jump, expands on hover
- **Pomodoro Timer** - subtle status bar with preset durations
- **Auto-save** with debounce and save status indicator
- **Undo Delete** - toast notification with 5-second undo window

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| Ctrl+N | New note |
| Ctrl+K | Quick switcher |
| Ctrl+B | Toggle sidebar |
| Ctrl+P | Toggle preview |
| Ctrl+Shift+F | Toggle focus mode |
| Ctrl+B (in editor) | Bold |
| Ctrl+I (in editor) | Italic |

## Running

```bash
dx serve --platform desktop
```
