BINS := $(filter-out run,$(MAKECMDGOALS))

dev:
	cd ./crates/sha-app && dx serve --platform web

run:
	@$(foreach bin,$(BINS),cargo run --bin $(bin) &)

%:
	@: