RUST_DIR := rust
DAY ?= 1
BIN_NAME := day$(shell printf "%02d" $(DAY))
FETCH_SCRIPT := scripts/fetch_input.sh

.PHONY: run fetch-input

run:
	cd $(RUST_DIR) && cargo run --bin $(BIN_NAME)

fetch-input:
	$(FETCH_SCRIPT) $(DAY)
