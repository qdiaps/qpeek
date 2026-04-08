# Contributing to qpeek
First off, thank you for considering contributing to `qpeek`! We welcome all contributions, from bug reports to new features.

## Local Development Setup
1. Ensure you have `Rust`, `Node.js` (v25+), `pnpm`, and `Docker` installed.
2. Clone the repository: `git clone https://github.com/qdiaps/qpeek.git`
3. Install dependencies: `pnpm install`
4. Start the dev environment: `make dev`

## Pull Request Process
1. **Branch Naming:** Use `feat/my-feature` or `fix/issue-description`.
2. **Commit Messages:** We strictly enforce [Conventional Commits](https://www.conventionalcommits.org/). Our Git hooks (Husky) will prevent you from making incorrectly formatted commits.
   * Example: `feat: add markdown table parser`
   * Example: `fix: resolve window flickering on Linux`
3. **Linting:** Always run `make lint` and `make fix` before committing. CI will fail if your code is not formatted properly.
4. **Draft PRs:** Feel free to open a Draft PR if you want feedback on incomplete work.

## Architectural Guidelines
* Keep the frontend "dumb". All heavy logic belongs in Rust.
* Avoid adding heavy dependencies unless absolutely necessary.
