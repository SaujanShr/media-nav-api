# media-nav-api

A self-hosted media navigation API backed by a plugin system and external provider servers.

---

## Prerequisites

| Tool | Purpose |
|------|---------|
| Rust / Cargo | Build the API server and plugins |
| Node.js 18+ & npm 9+ | Run provider servers |
| git | Clone provider libraries |

---

## Quick start

```sh
# 1. Set up providers (clone libs, install deps) — one-time step
make -C providers setup

# 2. Start providers
make -C providers start-consumet        # runs on port 4000 by default
# PORT=5000 make -C providers start-consumet   # custom port

# 3. Build and run the API server
make run
```

---

## Providers

Provider servers live under `providers/`. They are **not** managed by the root Makefile — use the dedicated `providers/Makefile` for full control.

See [`providers/README.md`](providers/README.md) for detailed setup instructions.

### Common provider commands

```sh
# First-time setup (clone external libs + npm install)
make -C providers setup

# Start the consumet provider server
make -C providers start-consumet

# Update consumet.ts to the latest upstream version
make -C providers update-consumet

# Stop consumet (kill by PID file)
make -C providers stop-consumet

# Remove cloned libs and node_modules
make -C providers clean
```

---

## API server

```sh
make build            # compile server + plugins
make build-plugins    # compile plugins only
make install-plugins  # copy + codesign plugins into plugins/
make run              # build, install plugins, then start the server
make clean            # remove build artefacts and installed plugins
```

---

## Plugins

Compiled plugin dylibs are placed in `plugins/` by `make install-plugins`. To add a new plugin create a new Cargo workspace member under `plugins/` and follow the existing `example` or `anime` plugin structure.
