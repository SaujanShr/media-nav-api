# Testing

## Running Tests

```sh
make test                # everything: plugin-sdk, plugins, server, providers
make test-plugin-sdk     # plugin-sdk only
make test-plugins        # wasm guest plugins only (e.g. plugins/example)
make test-server         # main Rust server only
make test-providers      # provider servers only (e.g. providers/example)
```

Each also runs directly with the underlying tool, without `make`:

```sh
cd plugin-sdk && cargo test --features guest
cd plugins/example && cargo test
cargo test                              # from the repo root, for the main server
cd providers/example && npm test
```

## Layout

### Rust (`plugin-sdk`, `plugins/*`, main server `src/`)

Unit tests live in a `tests/` submodule sitting *next to* the file it tests, not inside it.
For `src/validation.rs`, that's `src/tests/validation.rs`; for `src/auth/jwt.rs`, that's
`src/auth/tests/jwt.rs`. Each source file wires its test module in with:

```rust
#[cfg(test)]
#[path = "tests/validation.rs"]
mod tests;
```

placed near the top of the file, right after its `use` statements. This keeps test code out of
the implementation file while still compiling the test module as a genuine child module — so it
can see the parent's private items (functions, fields, etc.), which a true external integration
test (a top-level `tests/` directory at the crate root) cannot do.

Test files import explicitly from `super` — no `use super::*;` wildcards:

```rust
use super::{validate_password, validate_playlist_name, validate_username};
```

Only pure, self-contained logic is covered this way: validation rules, JWT round-tripping, the
playlist reordering math, error-type `Display` impls, HTTP error-response mapping, config parsing,
and the WASM-plugin enrichment transforms. Code that's inherently I/O-bound — database
repositories, migrations, the compiled WASM plugin pool, plugin registry loading from disk — isn't
unit-testable in isolation and isn't covered here; exercising it means running the full stack
(`make run`) and hitting it directly, e.g. via `./demo.sh`.

### JavaScript (`providers/*`)

Providers use Node's built-in test runner (`node --test`) and global `fetch` — no extra test
dependencies. Tests live under a top-level `test/` directory (the conventional JS layout, as
opposed to the Rust convention above):

```
providers/example/
└── test/
    └── server.test.js
```

Each test file spawns the provider as a real subprocess on a dedicated port, waits for `/health`,
exercises the HTTP endpoints with `fetch`, then kills the subprocess.

## Coverage by Area

| Area | What's covered |
|---|---|
| `plugin-sdk` | `PartialDate` ordering/formatting, guest utils (`filename_from_url`, `encode_query_param`, `push_attr!`), `PluginCallError` display, all query schema field validators (`search`, `number`, `date`, `filter`, `sort`) and their dispatch/defaulting |
| `plugins/example` | Pure transform functions in `enrich.rs` for every media/collection type (comic, album, TV series, movie series, gallery, book series) |
| main server (`src/`) | `validation.rs`, `auth/jwt.rs`, `auth/extractor.rs` (claims extraction and the bearer-token middleware), `services/playlist.rs`'s index-reordering math, `services::is_duplicate_key`, every handler's error-response mapping (`auth`, `library`, `media`, `playlist`, `plugin`, `settings`), `plugins/wasm.rs`'s `CallError` display, `Config::from_env` |
| `providers/example` | `/health`, `/items` pagination/search/sort, `/items/:id` (found and 404), unknown-route 404 |

## Adding Tests

- **New Rust module with logic worth testing:** add `#[cfg(test)] #[path = "tests/<name>.rs"] mod
  tests;` near the top of the file, and create the sibling `tests/<name>.rs` alongside it.
- **New provider:** add a `test/*.test.js` file following `providers/example/test/server.test.js`,
  and a `test` script in its `package.json` (`node --test`). Wire it into `providers/Makefile`'s
  `test`/`test-<name>` targets, following the `example` pattern.
