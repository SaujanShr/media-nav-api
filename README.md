# media-nav-api

Self-hosted media navigation API with plugin system and external provider servers.

> **New to the project?** Check out [SETUP.md](SETUP.md) for detailed installation instructions, troubleshooting, and development workflow.

## Quick Start

**Prerequisites:** Rust/Cargo, Node.js 18+, Docker

```sh
# 1. Clone and configure
git clone <repo-url>
cd media-nav-api
cp .env.example .env

# 2. Update JWT_SECRET in .env (required)
# Generate with: openssl rand -base64 32

# 3. Build and run everything
make run
```

The server starts at `http://localhost:8080`. 

**Detailed setup instructions:** See [SETUP.md](SETUP.md) for troubleshooting, custom credentials, and more.

### Custom Database Credentials

The default Docker setup uses `user`/`password` credentials. To use your own:

1. Create a `docker-compose.override.yml` file (gitignored):
   ```yaml
   services:
     postgres:
       environment:
         POSTGRES_USER: your_username
         POSTGRES_PASSWORD: your_password
   ```

2. Update `.env` to match:
   ```sh
   DATABASE_URL=postgresql://your_username:your_password@localhost:5433/media_nav_db
   ```

3. Start fresh (if the database was already running):
   ```sh
   make db-reset
   ```

Docker Compose automatically merges `docker-compose.override.yml` with `docker-compose.yml`, keeping your credentials local.

## Architecture

- **Server:** Rust/Actix-Web API with PostgreSQL database
- **Plugins:** Sandboxed WASM modules that extend content sources ([plugins/README.md](plugins/README.md))
- **Providers:** Standalone HTTP servers supplying media data ([providers/README.md](providers/README.md))
- **Database:** PostgreSQL 16 in Docker (port 5433 by default to avoid conflicts)

## Demo

Once everything is running, try the demo script:

```sh
# In another terminal (with server running)
./demo.sh
```

This demonstrates the full flow: register → login → fetch → enrich → get media.

## Testing

```sh
make test
```

Runs every test suite (plugin-sdk, plugins, server, providers). See [TESTING.md](TESTING.md) for
layout, per-area commands, and coverage details.

## Troubleshooting

### Port 5432 already in use

If you see "Bind for 0.0.0.0:5432 failed: port is already allocated", you already have PostgreSQL running. Options:

1. **Use the Docker setup on port 5433** (default): The setup automatically uses port 5433 to avoid conflicts
2. **Use your existing PostgreSQL**: Update `.env` to point to your existing database and skip `make db-start`

### Database connection errors

If the server can't connect to the database:

```sh
# Check database is running
docker compose ps postgres

# Check connection manually
make db-shell

# View database logs
make db-logs

# Reset everything (destructive)
make db-reset
```

### Provider not running

If the demo script reports providers aren't running:

```sh
# Check provider status
lsof -ti :4000

# View provider logs
tail -f /tmp/example-provider.log

# Restart providers
make stop-providers && make start-providers
```

## Contributing

**Add a plugin:** Create a standalone crate under `plugins/` targeting `wasm32-wasip1`, following the `example` plugin structure. See [plugins/README.md](plugins/README.md).

**Add a provider:** Create a subdirectory under `providers/` with HTTP endpoints. Add targets to `providers/Makefile`. See [providers/README.md](providers/README.md).

**Add tests:** See [TESTING.md](TESTING.md) for the test layout and conventions.
