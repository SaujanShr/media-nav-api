CARGO  := $(HOME)/.cargo/bin/cargo
TARGET := wasm32-wasip1

EXAMPLE_WASM := example/target/$(TARGET)/debug/example_plugin.wasm

.PHONY: help build build-example install install-example test test-example clean

help: ## Show this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-22s\033[0m %s\n", $$1, $$2}'

# ── Build ──────────────────────────────────────────────────────────────────────

build: build-example ## Build all plugin wasm modules

build-example: ## Build example plugin wasm module
	$(CARGO) build --manifest-path example/Cargo.toml --target $(TARGET)

# ── Install ────────────────────────────────────────────────────────────────────

install: install-example ## Copy compiled plugin wasm modules

install-example: build-example ## Install example plugin wasm module
	@mkdir -p .
	cp $(EXAMPLE_WASM) .

# ── Test ───────────────────────────────────────────────────────────────────────

test: test-example ## Run all plugin unit tests

test-example: ## Run example plugin unit tests
	$(CARGO) test --manifest-path example/Cargo.toml

# ── Maintenance ────────────────────────────────────────────────────────────────

clean: ## Remove installed plugins
	rm -f *.wasm
