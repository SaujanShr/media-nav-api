CARGO := $(HOME)/.cargo/bin/cargo

.PHONY: help run build setup start stop clean logs \
        build-plugins build-providers build-server \
        setup-providers start-providers stop-providers \
        clean-plugins clean-providers clean-server \
        db-start db-stop db-reset db-logs db-shell db-clean

help: ## Show this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-22s\033[0m %s\n", $$1, $$2}'

# ── Full Stack ─────────────────────────────────────────────────────────────────

run: build db-start start-providers ## Build everything, start database and providers, then run server
	@echo "→ Waiting for database to be ready..."
	@for i in $$(seq 1 30); do \
		if docker compose exec -T postgres pg_isready > /dev/null 2>&1; then \
			echo "→ Database ready"; \
			break; \
		fi; \
		sleep 1; \
	done
	$(CARGO) run --package media-nav-api

build: build-plugins build-providers build-server ## Build plugins, providers, and server

setup: setup-providers ## Set up all dependencies

start: db-start start-providers ## Start database and all background services

stop: stop-providers db-stop ## Stop all background services and database

clean: clean-plugins clean-providers clean-server ## Remove all build artifacts and dependencies

logs: ## Show logs from all services
	@echo "==> Database logs:"
	@docker compose logs postgres --tail=50
	@echo "\n==> Provider logs:"
	@$(MAKE) -C providers stop-example > /dev/null 2>&1 || true
	@tail -n 50 /tmp/example-provider.log 2>/dev/null || echo "No provider logs found"

# ── Plugins ────────────────────────────────────────────────────────────────────

build-plugins: ## Build and install all plugin dylibs
	$(MAKE) -C plugins build install

clean-plugins: ## Remove installed plugins
	$(MAKE) -C plugins clean

# ── Providers ──────────────────────────────────────────────────────────────────

build-providers: ## Build all providers (currently a no-op, providers are Node.js)
	@echo "→ Providers are Node.js-based, no build step required"

setup-providers: ## Set up all provider dependencies
	$(MAKE) -C providers setup

start-providers: ## Start all provider servers in the background
	$(MAKE) -C providers start

stop-providers: ## Stop all provider servers
	$(MAKE) -C providers stop-example

clean-providers: ## Remove all provider artifacts and dependencies
	$(MAKE) -C providers clean

# ── Server ─────────────────────────────────────────────────────────────────────

build-server: build-plugins ## Build the main Rust server
	$(CARGO) build --package media-nav-api

clean-server: ## Remove Rust build artifacts
	cargo clean

# ── Database ───────────────────────────────────────────────────────────────────

db-start: ## Start PostgreSQL database in Docker
	@if docker compose ps postgres | grep -q "Up"; then \
		echo "→ Database already running"; \
	else \
		echo "→ Starting PostgreSQL database..."; \
		docker compose up -d postgres; \
		echo "→ Waiting for database to be ready..."; \
		for i in $$(seq 1 30); do \
			if docker compose exec -T postgres pg_isready > /dev/null 2>&1; then \
				echo "→ Database ready"; \
				break; \
			fi; \
			sleep 1; \
		done; \
	fi

db-stop: ## Stop PostgreSQL database
	@echo "→ Stopping PostgreSQL database..."
	docker compose stop postgres

db-reset: ## Stop, remove, and restart the database (WARNING: deletes all data)
	@echo "→ Resetting database (all data will be lost)..."
	docker compose down -v postgres
	$(MAKE) db-start

db-logs: ## Show PostgreSQL logs
	docker compose logs -f postgres

db-shell: ## Open a psql shell to the database
	docker compose exec postgres psql -U user -d media_nav_db

db-clean: ## Remove database container and volumes (WARNING: deletes all data)
	@echo "→ Removing database container and volumes..."
	docker compose down -v postgres
