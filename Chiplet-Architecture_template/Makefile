CARGO ?= cargo

.PHONY: all build run test fmt lint doc clean

all: build

build:
	$(CARGO) build

run:
	$(CARGO) run --bin chiplet-architecture

test:
	$(CARGO) test --all

fmt:
	$(CARGO) fmt --all

lint:
	$(CARGO) clippy --all-targets --all-features -- -D warnings

doc:
	$(CARGO) doc --no-deps

clean:
	$(CARGO) clean
