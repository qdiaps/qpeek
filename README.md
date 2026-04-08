# qpeek ⚡

[![PR Checks](https://github.com/qdiaps/qpeek/actions/workflows/pr-checks.yml/badge.svg)](https://github.com/qdiaps/qpeek/actions/workflows/pr-checks.yml)
![OS: Linux](https://img.shields.io/badge/OS-Linux-blue?logo=linux)
![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)

A modern, blazing fast cheat-sheet viewer built with Tauri and Vue.

## Sneak Peek

_Screenshots and GIFs are coming soon..._

## Features

- **Interactive Node Graph:** Renders your markdown files into a beautiful, infinite canvas using Vue Flow.
- **Blazing Fast:** Built on Rust and Tauri for instant startup and minimal memory footprint.
- **Zero-Config by Default:** Works perfectly out of the box, but is highly customizable.

## Roadmap (TODO)

- [ ] **Daemon-Client Architecture:** Background process for zero-latency window toggling.
- [ ] **Hot-Reload:** Instantly updates the UI when the underlying markdown file is saved.
- [ ] **Click-to-Copy:** One-click copying of code snippets directly from the graph.
- [ ] **Dynamic Placeholders:** Support for variables in cheat-sheets.

## Installation

Currently, `qpeek` is officially supported on **Linux**.

### Option 1: Pre-built Binaries

Download the latest `.AppImage` or `.deb` from the [GitHub Releases](https://github.com/qdiaps/qpeek/releases) page.

### Option 2: Build from Source

Ensure you have `Rust`, `Node.js` (v25+), `pnpm`, and `Docker` installed.

```bash
git clone [https://github.com/qdiaps/qpeek.git](https://github.com/qdiaps/qpeek.git)
cd qpeek

# Build an isolated Linux release binary using Docker
make docker-build
```

## Usage

Simply pass a markdown file to the viewer:

```bash
qpeek my_cheatsheet.md
```

## Documentation

Want to dive deeper? Check out our docs/ folder:

- [CLI Reference](https://github.com/qdiaps/qpeek/blob/master/docs/CLI.md) - Available commands and flags.
- [Configuration](https://github.com/qdiaps/qpeek/blob/master/docs/CONFIG.md) - How to customize the viewer.
- [FAQ](https://github.com/qdiaps/qpeek/blob/master/docs/FAQ.md) - Common questions and error fixes.

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/qdiaps/qpeek/blob/master/LICENSE) file for details.
