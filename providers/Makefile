CONSUMET_DIR  := consumet
CONSUMET_LIB  := $(CONSUMET_DIR)/consumet.ts
CONSUMET_REPO := https://github.com/prince-ao/consumet.ts
CONSUMET_PID  := .consumet.pid
CONSUMET_LOG  := /tmp/consumet.log
NPM           := npm --registry https://registry.npmjs.org

.PHONY: help setup setup-consumet clone-consumet install-consumet install-server \
        start-consumet stop-consumet update-consumet clean-consumet clean

help: ## Show this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-22s\033[0m %s\n", $$1, $$2}'

# ── Setup ──────────────────────────────────────────────────────────────────────

setup: setup-consumet ## Set up all providers

setup-consumet: clone-consumet install-consumet install-server ## Clone consumet.ts and install all dependencies

clone-consumet: ## Clone the consumet.ts library (skipped if already present)
	@if [ ! -d "$(CONSUMET_LIB)" ]; then \
		echo "→ Cloning consumet.ts..."; \
		git clone $(CONSUMET_REPO) $(CONSUMET_LIB); \
	else \
		echo "→ consumet.ts already cloned, skipping."; \
	fi

install-consumet: ## Install consumet.ts npm dependencies
	@echo "→ Installing consumet.ts dependencies..."
	cd $(CONSUMET_LIB) && $(NPM) install

install-server: ## Install consumet server npm dependencies
	@echo "→ Installing consumet server dependencies..."
	cd $(CONSUMET_DIR) && $(NPM) install

# ── Run ────────────────────────────────────────────────────────────────────────

start-consumet: ## Start the consumet provider server in the background (PORT=4000 by default)
	@if [ -f $(CONSUMET_PID) ] && kill -0 $$(cat $(CONSUMET_PID)) 2>/dev/null; then \
		echo "→ consumet already running (pid $$(cat $(CONSUMET_PID)))"; \
	else \
		echo "→ Starting consumet server on port $(PORT) (log: $(CONSUMET_LOG))..."; \
		cd $(CONSUMET_DIR) && PORT=$(PORT) $(NPM) start > $(CONSUMET_LOG) 2>&1 & \
		echo $$! > ../$(CONSUMET_PID); \
		echo "→ consumet started (pid $$(cat ../$(CONSUMET_PID)))"; \
	fi

stop-consumet: ## Stop the consumet provider server
	@if [ -f $(CONSUMET_PID) ] && kill -0 $$(cat $(CONSUMET_PID)) 2>/dev/null; then \
		kill $$(cat $(CONSUMET_PID)) && echo "→ consumet stopped"; \
		rm -f $(CONSUMET_PID); \
	else \
		echo "→ consumet is not running"; \
		rm -f $(CONSUMET_PID); \
	fi

# ── Maintenance ────────────────────────────────────────────────────────────────

update-consumet: ## Pull the latest consumet.ts changes
	@echo "→ Updating consumet.ts..."
	cd $(CONSUMET_LIB) && git pull
	cd $(CONSUMET_LIB) && $(NPM) install

clean-consumet: ## Remove cloned consumet.ts and node_modules
	@echo "→ Cleaning consumet..."
	rm -rf $(CONSUMET_LIB)
	rm -rf $(CONSUMET_DIR)/node_modules

clean: clean-consumet ## Remove all generated/cloned artifacts

