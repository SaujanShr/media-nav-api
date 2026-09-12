# Providers

Standalone HTTP servers that supply media data to plugins. Can be written in any language.

## Example Provider

Node.js/Express server serving dummy data from JSON fixtures. Template for new providers.

**Endpoints:**
- `GET /items` - list media items, with pagination (`page`, `limit`), search (`search`, substring match on title), and sort (`sort=title`, `direction=asc|desc`)
- `GET /items/:id` - get item details

**Environment:** 
- `PORT` (default: 4000)

**Custom port:**
```sh
PORT=5000 make start-example
```

Note: If you change the port, update your plugin's fetch/enrich logic to use the new port.

## Creating a Provider

1. **Create directory:** `providers/my-provider/`
2. **Implement HTTP server** with endpoints plugins will call
3. **Add Makefile targets to `providers/Makefile`** (follow the `example` pattern):
   ```makefile
   MY_PROVIDER_DIR := my-provider
   MY_PROVIDER_PID := .my-provider.pid
   MY_PROVIDER_LOG := /tmp/my-provider.log

   setup-my-provider: install-my-provider
   
   install-my-provider:
       @echo "→ Installing my-provider dependencies..."
       cd $(MY_PROVIDER_DIR) && npm install

   run-my-provider: clean-my-provider setup-my-provider start-my-provider

   start-my-provider:
       @PORT=$${PORT:-4001}; \
       PID=$$(lsof -ti :$$PORT); \
       if [ -n "$$PID" ]; then \
           echo "→ my-provider already running on port $$PORT (pid $$PID)"; \
       else \
           echo "→ Starting my-provider on port $$PORT (log: $(MY_PROVIDER_LOG))..."; \
           cd $(MY_PROVIDER_DIR) && PORT=$$PORT npm start > $(MY_PROVIDER_LOG) 2>&1 & \
           echo $$! > ../$(MY_PROVIDER_PID); \
           echo "→ my-provider started (pid $$(cat ../$(MY_PROVIDER_PID)))"; \
       fi

   stop-my-provider:
       @PORT=$${PORT:-4001}; \
       PID=$$(lsof -ti :$$PORT); \
       if [ -n "$$PID" ]; then \
           kill $$PID && echo "→ my-provider stopped (pid $$PID)"; \
           rm -f $(MY_PROVIDER_PID); \
       else \
           echo "→ my-provider is not running"; \
           rm -f $(MY_PROVIDER_PID); \
       fi

   clean-my-provider: stop-my-provider
       @echo "→ Cleaning my-provider..."
       rm -rf $(MY_PROVIDER_DIR)/node_modules
       rm -f $(MY_PROVIDER_PID)
   ```
   Then update the `setup`, `start`, `run`, `stop`, and `clean` targets to include your provider.
4. **Guidelines:**
   - Stateless and horizontally scalable
   - Return proper HTTP status codes
   - Use consistent JSON response format
   - Port-based process detection prevents duplicate starts
   - Include test fixtures
   - Document all endpoints
