# Example Provider

Minimal Node.js/Express provider serving dummy data from JSON fixtures. Reference implementation for new providers.

## Setup

```sh
# From providers/ directory
make setup-example

# Or manually
npm install
```

## Running

```sh
# From providers/ directory
make start-example

# Or manually
npm start              # default port 4000
PORT=5000 npm start    # custom port
```

## Endpoints

**`GET /health`**
```json
{"status": "ok", "provider": "example"}
```

**`GET /items?page=1&limit=10`**
```json
{
  "items": [
    {"id": "example-item-1", "title": "Example Item 1", "thumbnail_url": "..."}
  ],
  "total": 3,
  "page": 1,
  "limit": 10
}
```

**`GET /items/:id`**
```json
{
  "id": "example-item-1",
  "version": "1.0",
  "metadata": {"title": "...", "subtitle": "...", "summary": "...", "attributes": [...]},
  "resources": {"thumbnail": "...", "preview": [...], "media": {...}}
}
```

## Fixtures

Edit `fixtures/items.json` and `fixtures/details.json` to customize test data.

## Integration

Used by `plugins/example/`. Start both the provider and API server to test full integration.