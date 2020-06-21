.PHONY: help flamegraph geiger tree udeps crev audit deny bloat

help: ## Show this help.
	@perl -e '$(HELP_FUN)' $(MAKEFILE_LIST)

MIRI_NIGHTLY=nightly-$(curl -s https://rust-lang.github.io/rustup-components-history/x86_64-unknown-linux-gnu/miri)
miri: ## Run tests
	echo "Installing latest nightly with Miri: $$MIRI_NIGHTLY"
	rustup set profile minimal
	rustup default "$$MIRI_NIGHTLY"
	rustup component add miri
	cargo clean
	cargo miri test
	cargo clean
	# cargo miri run

flamegraph: ## flamegraph
	cargo install flamegraph
	cargo flamegraph

geiger: ## Warn about unsafe
	cargo install cargo-geiger
	cargo geiger

tree: ## Dependency tree
	cargo tree

fmt: ## __
	cargo fmt

udeps: ## Unused dependencies
	cargo install cargo-udeps --locked
	cargo udeps

crev: ## ??
	cargo install cargo-crev
	cargo crev

audit: ## ??
	cargo install cargo-audit
	cargo audit

deny: ## Deny crates
	cargo install cargo-deny
	cargo deny init
	cargo deny check

bloat: ## Why is my executable so big ?
	cargo install cargo-bloat
	cargo bloat

release: ## Release
	cargo install cargo-release
	cargo release --dry-run


.DEFAULT_GOAL = help
SHELL := /bin/bash

GREEN := $(shell command -v tput >/dev/null 2>&1 && tput -Txterm setaf 2 || echo "")
YELLOW := $(shell command -v tput >/dev/null 2>&1 && tput -Txterm setaf 3 || echo "")
RED := $(shell command -v tput >/dev/null 2>&1 && tput -Txterm setaf 1 || echo "")
RESET := $(shell command -v tput >/dev/null 2>&1 && tput -Txterm sgr0 || echo "")

HELP_FUN = %help; \
	while(<>) { push @{$$help{$$2 // "Other"}}, [$$1, $$3] if /^([a-zA-Z\-._]+)\s*:.*\#\#(?:@([a-zA-Z\-_]+))?\s(.*)$$/ }; \
	print "$(RESET)project: $(PURPLE)$(NAME)$(RESET)\n"; \
	print "usage: make [target]\n\n"; \
	for (sort keys %help) { \
	print "$$_:\n"; \
	for (@{$$help{$$_}}) { \
	$$sep = " " x (25 - length $$_->[0]); \
	print " ${YELLOW}$$_->[0]${RESET}$$sep${GREEN}$$_->[1]${RESET}\n"; \
	}; \
	print "\n"; }
