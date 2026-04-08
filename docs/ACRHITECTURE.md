# Architecture
`qpeek` is a modern, blazing fast cheat-sheet viewer built with Tauri and Vue.

## Tech Stack
- **Frontend:** Vue 3, TypeScript, Tailwind CSS v4, Vue Flow.
- **Backend:** Rust, Tauri.
- **Communication:** Tauri IPC (Inter-Process Communication).

## Core Modules
1. **Daemon:** Runs in the background with minimal footprint.
2. **Markdown Parser:** Transforms local markdown files into a node-based interactive graph.
3. **UI Renderer:** A lightweight, reactive layer that receives pre-parsed data via IPC and instantly draws it on an interactive canvas.
