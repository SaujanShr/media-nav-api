CARGO := $(HOME)/.cargo/bin/cargo
EXT   := $(if $(filter Darwin,$(shell uname -s)),dylib,so)

EXAMPLE_LIB := ../target/debug/libexample_plugin.$(EXT)

.PHONY: help build build-example install install-example clean

help: ## Show this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-22s\033[0m %s\n", $$1, $$2}'

# ── Build ──────────────────────────────────────────────────────────────────────

build: build-example ## Build all plugin dylibs

build-example: ## Build example plugin dylib
	$(CARGO) build --package example-plugin

# ── Install ────────────────────────────────────────────────────────────────────

install: install-example ## Copy compiled plugin dylibs and sign them

install-example: build-example ## Install example plugin dylib
	@mkdir -p .
	cp $(EXAMPLE_LIB) .
	codesign -s - libexample_plugin.$(EXT)

# ── Maintenance ────────────────────────────────────────────────────────────────

clean: ## Remove installed plugins
	rm -f *.dylib *.so
