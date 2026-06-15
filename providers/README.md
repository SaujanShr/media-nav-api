# Providers

Standalone HTTP servers that supply media data to plugins. Can be written in any language.

## Setup

```sh
# Install all provider dependencies
make setup

# Or individually
make setup-example
```

## Running

```sh
# Start example provider (port 4000)
make start-example

# Custom port
PORT=5000 make start-example

# Stop
make stop-example

# Clean dependencies
make clean
```

## Example Provider

Node.js/Express server serving dummy data from JSON fixtures. Template for new providers.

**Endpoints:**
- `GET /items` - list media items (with pagination)
- `GET /items/:id` - get item details

**Environment:** `PORT` (default: 4000)

## Creating a Provider

1. **Create directory:** `providers/my-provider/`
2. **Implement HTTP server** with endpoints plugins will call
3. **Add Makefile targets:**
   ```makefile
   setup-my-provider:
       cd my-provider && npm install

   start-my-provider:
       cd my-provider && PORT=$(PORT) npm start &
   ```
4. **Guidelines:**
   - Stateless and horizontally scalable
   - Return proper HTTP status codes
   - Use consistent JSON response format
   - Include test fixtures
   - Document all endpoints
