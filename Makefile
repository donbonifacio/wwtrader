help: ## Shows the available commands.
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

run: ## Runs the game.
	cargo run -p ggez-fe

test: ## Runs the test suite.
	RUST_BACKTRACE=1 cargo test

lint: ## Checks lint rules.
	cargo fmt --all -- --check
	cargo clippy --verbose --all-targets --all-features -- -D warnings

