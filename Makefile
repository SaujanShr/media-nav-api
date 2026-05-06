CARGO       := $(HOME)/.cargo/bin/cargo
PLUGINS_DIR := plugins
EXT         := $(if $(filter Darwin,$(shell uname -s)),dylib,so)

EXAMPLE_LIB := target/debug/libexample_plugin.$(EXT)
ANIME_LIB   := target/debug/libanime_plugin.$(EXT)

.PHONY: run build build-plugins install-plugins clean

## Build everything and start the server
run: build install-plugins
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

## Remove build artefacts and installed plugins
clean:
	cargo clean
	rm -f $(PLUGINS_DIR)/*.dylib $(PLUGINS_DIR)/*.so

