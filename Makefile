.PHONY: dev docs

dev:
	cd ./crates/sha-app && dx serve --platform web

dev-cli:
	cargo run

docs:
	cd ./docs && bun dev