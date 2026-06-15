# media-nav-api

Self-hosted media navigation API with plugin system and external provider servers.

## Quick Start

**Prerequisites:** Rust/Cargo, Node.js 18+, PostgreSQL

```sh
# 1. Configure environment
cp .env.example .env
# Edit .env: set DATABASE_URL, JWT_SECRET (32+ chars), PLUGINS_DIR, HOST, PORT

# 2. Set up and start providers
make -C providers setup
make -C providers start-example    # runs on port 4000

# 3. Build and run API
make run
```

## Commands

```sh
# API server
make run              # build, install plugins, start server
make build            # compile server + plugins
make build-plugins    # plugins only
make install-plugins  # copy plugins to plugins/ dir
make clean            # remove build artifacts

# Providers (see providers/README.md)
make -C providers setup          # install dependencies
make -C providers start-example  # start example provider
make -C providers stop-example   # stop example provider

# Optional: enable debug logging
export RUST_LOG=media_nav_api=debug,actix_web=info
```

## Contributing

**Add a plugin:** Create a Cargo workspace member under `plugins/` following the `example` plugin structure. See [plugins/README.md](plugins/README.md).

**Add a provider:** Create a subdirectory under `providers/` with HTTP endpoints. Add targets to `providers/Makefile`. See [providers/README.md](providers/README.md).
