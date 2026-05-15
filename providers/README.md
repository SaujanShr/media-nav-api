# Providers

This directory contains external provider servers that supply media data to the main API.

---

## consumet

A lightweight Express server that wraps the [consumet.ts](https://github.com/consumet/consumet.ts) library to expose anime episode listings and streaming source endpoints.

### Dependencies

| Tool | Min version |
|------|-------------|
| Node.js | 18+ |
| npm | 9+ |
| git | any recent |

### Setup

The `consumet.ts` library is **not bundled** in this repository. You must clone it before starting the server.

Use the provided Makefile from this directory:

```sh
# Clone consumet.ts and install all npm dependencies
make setup

# Start the consumet server (default port: 4000)
make start-consumet

# Or set a custom port
PORT=5000 make start-consumet
```

Alternatively, run the steps manually:

```sh
# 1. Clone consumet.ts into the expected location
git clone https://github.com/consumet/consumet.ts providers/consumet/consumet.ts

# 2. Install consumet.ts dependencies
cd providers/consumet/consumet.ts && npm install && cd -

# 3. Install server dependencies
cd providers/consumet && npm install && cd -

# 4. Start the server
cd providers/consumet && npm start
```

### Endpoints

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/anime/:malId/episodes` | Fetch episode list for an anime by MAL ID |
| `GET` | `/episode/sources?episodeId=<id>` | Fetch streaming sources for an episode |

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | `4000` | Port the consumet server listens on |

---

## Adding a new provider

1. Create a subdirectory under `providers/` (e.g. `providers/my-provider/`).
2. Add a `setup` target and a `start-<name>` target to the root `providers/Makefile`.
3. Document it in a new section above.

