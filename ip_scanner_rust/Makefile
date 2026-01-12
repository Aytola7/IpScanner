# Makefile for IP Scanner Rust

# Variables
CARGO = cargo
BINARY_NAME = ip_scanner_rust

# Build targets
.PHONY: all build release check clean run test help

all: check

## build: Build debug version
build:
	$(CARGO) build

## release: Build optimized release version
release:
	$(CARGO) build --release

## check: Check code for errors without building
check:
	$(CARGO) check

## test: Run tests
test:
	$(CARGO) test

## clean: Remove build artifacts
clean:
	$(CARGO) clean
	rm -f safePing.txt safeSocketConnect.txt app.log

## run: Run with default settings (100 workers, 30 concurrency)
run: build
	$(CARGO) run

## run-release: Run optimized release version
run-release: release
	$(CARGO) run --release

## run-example: Run with example IP file
run-example: build
	$(CARGO) run -- --ip-file example_ips.txt --workers 10 --concurrency 5

## run-fast: Run with fewer workers for faster testing
run-fast: build
	$(CARGO) run -- --workers 20 --concurrency 10

## format: Format code with rustfmt
format:
	$(CARGO) fmt

## lint: Run clippy linter
lint:
	$(CARGO) clippy -- -D warnings

## install: Install binary to system
install: release
	$(CARGO) install --path .

## help: Show this help
help:
	@echo "Available targets:"
	@grep -E '^##' Makefile | sed 's/^## /  /'
