CARGO        := $(HOME)/.cargo/bin/cargo
PLUGINS_DIR  := plugins
EXT          := $(if $(filter Darwin,$(shell uname -s)),dylib,so)

EXAMPLE_LIB  := target/debug/libexample_plugin.$(EXT)
ANIME_LIB    := target/debug/libanime_plugin.$(EXT)

CONSUMET_CTR  := consumet
CONSUMET_PORT := 4000
CONSUMET_PID  := .consumet.pid

.PHONY: run build build-plugins install-plugins consumet stop-consumet clean

## Build everything, start consumet, and start the server
run: build install-plugins consumet
	$(CARGO) run --package media-nav-web

## Build the server and all plugins
build: build-plugins
	$(CARGO) build --package media-nav-web

## Build all plugin dylibs
build-plugins:
	$(CARGO) build --package example-plugin
	$(CARGO) build --package anime-plugin

## Copy compiled plugin dylibs into the plugins directory and sign them
install-plugins: build-plugins
	@mkdir -p $(PLUGINS_DIR)
	cp $(EXAMPLE_LIB) $(PLUGINS_DIR)/
	codesign -s - $(PLUGINS_DIR)/libexample_plugin.$(EXT)
	cp $(ANIME_LIB) $(PLUGINS_DIR)/
	codesign -s - $(PLUGINS_DIR)/libanime_plugin.$(EXT)

## Start consumet server (no-op if already running)
consumet:
	@if [ -f $(CONSUMET_PID) ] && kill -0 $$(cat $(CONSUMET_PID)) 2>/dev/null; then \
		echo "consumet already running (pid $$(cat $(CONSUMET_PID)))"; \
	else \
		cd providers && npm install --silent && cd consumet && npm install --silent && cd .. && PORT=$(CONSUMET_PORT) npm start > /tmp/consumet.log 2>&1 & \
		echo $$! > $(CONSUMET_PID); \
		echo "consumet started (pid $$(cat $(CONSUMET_PID)))"; \
	fi

## Stop consumet
stop-consumet:
	@if [ -f $(CONSUMET_PID) ]; then \
		kill $$(cat $(CONSUMET_PID)) 2>/dev/null && echo "consumet stopped" || true; \
		rm -f $(CONSUMET_PID); \
	fi

## Remove build artefacts, installed plugins, and stop consumet
clean: stop-consumet
	cargo clean
	rm -f $(PLUGINS_DIR)/*.dylib $(PLUGINS_DIR)/*.so

