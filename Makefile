.PHONY: help build run test lint fmt fmt-check clean docker-build docker-up docker-down docker-clean migrate migrate-revert sqlx-prepare check watch 

# self-documenting help target — a common Makefile pattern that turns your ## comments into a auto-generated menu,
help:
	@grep -E '^[a-zA-Z_-]+:.*?##' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

run: ## Run in dev mode
	cargo run

watch: ## Auto-reload on change (requires cargo-watch)
	cargo watch -x run

build: ## Build the application
	cargo build --release

test: ## Run all tests
	SQLX_OFFLINE=true cargo test -- --nocapture

check: ## Fast type-check without codegen
	cargo check --all-targets

lint: ## Run clippy, deny warnings
	cargo clippy --all-targets --all-features -- -D warnings