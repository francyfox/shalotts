.PHONY: dev docs component-add component-remove component-list

dev:
	dx serve --package sha_web

dev-cli:
	cargo run

docs:
	cd ./docs && bun dev

# Dx components shortcuts
component-add:
	@if [ -z "$(c)" ]; then \
		echo "Usage: make component-add c=<component-name>"; \
		echo "Example: make component-add c=button"; \
		exit 1; \
	fi
	dx components add $(c) --module-path crates/sha-web/src/components --global-assets-path crates/sha-web/assets

component-remove:
	@if [ -z "$(c)" ]; then \
		echo "Usage: make component-remove c=<component-name>"; \
		exit 1; \
	fi
	rm -rf crates/sha-web/src/components/$(c)
	@echo "Don't forget to remove 'pub mod $(c);' from crates/sha-web/src/components/mod.rs"

component-list:
	dx components list
