EXAMPLE_DIR  := example
EXAMPLE_PID  := .example.pid
EXAMPLE_LOG  := /tmp/example-provider.log
NPM          := npm --registry https://registry.npmjs.org

.PHONY: help setup setup-example install-example \
        start-example stop-example clean-example clean

help: ## Show this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-22s\033[0m %s\n", $$1, $$2}'

# ── Setup ──────────────────────────────────────────────────────────────────────

setup: setup-example ## Set up all providers

setup-example: install-example ## Install example provider dependencies

install-example: ## Install example provider npm dependencies
	@echo "→ Installing example provider dependencies..."
	cd $(EXAMPLE_DIR) && $(NPM) install

# ── Run ────────────────────────────────────────────────────────────────────────

start-example: ## Start the example provider server in the background (PORT=4000 by default)
	@if [ -f $(EXAMPLE_PID) ] && kill -0 $$(cat $(EXAMPLE_PID)) 2>/dev/null; then \
		echo "→ example provider already running (pid $$(cat $(EXAMPLE_PID)))"; \
	else \
		echo "→ Starting example provider on port $${PORT:-4000} (log: $(EXAMPLE_LOG))..."; \
		cd $(EXAMPLE_DIR) && PORT=$${PORT:-4000} $(NPM) start > $(EXAMPLE_LOG) 2>&1 & \
		echo $$! > ../$(EXAMPLE_PID); \
		echo "→ example provider started (pid $$(cat ../$(EXAMPLE_PID)))"; \
	fi

stop-example: ## Stop the example provider server
	@if [ -f $(EXAMPLE_PID) ] && kill -0 $$(cat $(EXAMPLE_PID)) 2>/dev/null; then \
		kill $$(cat $(EXAMPLE_PID)) && echo "→ example provider stopped"; \
		rm -f $(EXAMPLE_PID); \
	else \
		echo "→ example provider is not running"; \
		rm -f $(EXAMPLE_PID); \
	fi

# ── Maintenance ────────────────────────────────────────────────────────────────

clean-example: ## Remove example provider node_modules
	@echo "→ Cleaning example provider..."
	rm -rf $(EXAMPLE_DIR)/node_modules
	rm -f $(EXAMPLE_PID)

clean: clean-example ## Remove all generated artifacts
