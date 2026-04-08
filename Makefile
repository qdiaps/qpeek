.PHONY: dev lint check fix build docker-build

dev:
	pnpm run tauri dev

lint:
	pnpm exec eslint .
	pnpm exec prettier --check .
	cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
	cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings

fix:
	pnpm exec eslint --fix .
	pnpm exec prettier --write .
	cargo fmt --manifest-path src-tauri/Cargo.toml
	@echo "All fixes applied!"

build:
	pnpm run tauri build

docker-build:
	docker build -t qpeek-builder -f Dockerfile.build .
	docker run --rm -v $(PWD):/app qpeek-builder pnpm run tauri build
