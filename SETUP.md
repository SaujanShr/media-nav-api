# Setup Guide

Detailed setup instructions for media-nav-api.

## Prerequisites

Install the following before proceeding:

### 1. Rust and Cargo

```sh
# Install via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Verify installation
cargo --version
```

### 2. Node.js 18+

```sh
# Install via nvm (recommended)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 18
nvm use 18

# Or install via package manager
# macOS: brew install node
# Ubuntu: sudo apt install nodejs npm

# Verify installation
node --version
npm --version
```

### 3. Docker

- **macOS/Windows:** Install [Docker Desktop](https://www.docker.com/products/docker-desktop)
- **Linux:** Install Docker Engine and Docker Compose

```sh
# Verify installation
docker --version
docker compose version
```

## Installation

### Step 1: Clone the Repository

```sh
git clone <repo-url>
cd media-nav-api
```

### Step 2: Configure Environment

Copy the example environment file:

```sh
cp .env.example .env
```

The default `.env.example` contains:

```sh
DATABASE_URL=postgresql://user:password@localhost:5433/media_nav_db
JWT_SECRET=your-secret-key-here-min-32-chars
PLUGINS_DIR=./plugins
HOST=127.0.0.1
PORT=8080
```

**Important:** Change `JWT_SECRET` to a secure random string (minimum 32 characters):

```sh
# Generate a secure secret (macOS/Linux)
openssl rand -base64 32
```

Then update `.env`:

```sh
JWT_SECRET=<your-generated-secret>
```

### Step 3: (Optional) Custom Database Credentials

If you want to use credentials other than `user`/`password`:

1. Create `docker-compose.override.yml`:

```yaml
services:
  postgres:
    environment:
      POSTGRES_USER: your_username
      POSTGRES_PASSWORD: your_password
```

2. Update `DATABASE_URL` in `.env`:

```sh
DATABASE_URL=postgresql://your_username:your_password@localhost:5433/media_nav_db
```

**Note:** `docker-compose.override.yml` is gitignored and won't be committed.

### Step 4: Build and Run

```sh
make run
```

This single command:
1. Builds all plugins and installs them
2. Installs provider dependencies
3. Starts PostgreSQL database in Docker
4. Starts provider servers in the background
5. Runs database migrations
6. Starts the API server

The server will be available at `http://localhost:8080`.

## Verification

### 1. Check Server Health

```sh
curl http://localhost:8080/health
```

Expected output:
```json
{"status":"healthy"}
```

### 2. Check Database Connection

```sh
make db-shell
```

This opens a `psql` shell. Try:

```sql
\dt  -- list tables
\q   -- quit
```

### 3. Check Providers

```sh
curl http://localhost:4000/items
```

Should return a JSON array of media items.

### 4. Run the Demo

```sh
./demo.sh
```

This walks through the complete flow: register → login → fetch → enrich → get media.

## Development Workflow

### Starting the Stack

```sh
# Full stack (database + providers + server)
make run

# Just database and providers (run server separately)
make start
cargo run
```

### Stopping Services

```sh
# Stop providers and database
make stop

# Stop just providers
make stop-providers

# Stop just database
make db-stop
```

### Rebuilding

```sh
# Rebuild everything
make build

# Rebuild just plugins
make build-plugins

# Rebuild just server
make build-server
```

### Cleaning Up

```sh
# Clean all build artifacts (keeps database data)
make clean

# Clean and remove database data (destructive)
make clean && make db-clean
```

### Viewing Logs

```sh
# All logs (database + providers)
make logs

# Just database logs
make db-logs

# Just provider logs
tail -f /tmp/example-provider.log

# Server logs (if running via make run)
# Logs appear in the terminal
```

## Common Issues

### Port Conflicts

**Problem:** Port 5432 or 5433 already in use

**Solution:**
```sh
# Check what's using the port
lsof -ti :5432
lsof -ti :5433

# Option 1: Use your existing PostgreSQL
# Update .env to point to it and skip `make db-start`

# Option 2: Change Docker port
# Edit docker-compose.yml and change "5433:5432" to another port
# Update DATABASE_URL in .env accordingly
```

### Provider Port Conflicts

**Problem:** Port 4000 already in use

**Solution:**
```sh
# Start provider on different port
PORT=5000 make start-providers

# Then configure your plugin to use port 5000
```

### Database Connection Refused

**Problem:** Server can't connect to database

**Solutions:**
```sh
# 1. Check database is running
docker compose ps postgres

# 2. Check credentials match
cat .env | grep DATABASE_URL
docker compose exec postgres env | grep POSTGRES

# 3. Reset database (if credentials changed)
make db-reset

# 4. Check migrations ran
docker compose exec postgres psql -U <user> -d media_nav_db -c "\dt"
```

### Plugin Not Loading

**Problem:** "Plugin not found" or "Failed to load plugin"

**Solutions:**
```sh
# 1. Check plugin was built and installed
ls -la plugins/*.dylib plugins/*.so

# 2. Rebuild and install
make build-plugins

# 3. Check permissions (macOS)
codesign -dv plugins/libexample_plugin.dylib

# 4. Check PLUGINS_DIR in .env
cat .env | grep PLUGINS_DIR
```

### Migrations Failed

**Problem:** Database migrations don't run

**Solutions:**
```sh
# 1. Check migration files exist
ls -la migrations/

# 2. Manually run migrations (if server isn't running)
docker compose exec postgres psql -U <user> -d media_nav_db -f /docker-entrypoint-initdb.d/0001_create_users.sql

# 3. Reset database and let migrations run fresh
make db-reset
make run
```

## Next Steps

- **Add a plugin:** See [plugins/README.md](plugins/README.md)
- **Add a provider:** See [providers/README.md](providers/README.md)
- **API Documentation:** Check the `/api-docs` endpoint (if enabled)
- **Run tests:** `cargo test`

## Getting Help

- Check the [main README](README.md) for command reference
- Review component-specific READMEs in `plugins/` and `providers/`
- Check logs with `make logs` or `make db-logs`
- Open an issue on GitHub if you encounter problems
