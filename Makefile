.PHONY: dev docs

dev:
	dx serve --package sha_web

dev-cli:
	cargo run

docs:
	cd ./docs && bun dev
