BUILD_ARGS ?= --release
RELEASE_ARGS ?= --dry-run
LINT_ARGS ?= --all-targets --all-features -- -D warnings

build:
	cargo build $(BUILD_ARGS)

test:
	cargo test

release:
	cargo publish $(RELEASE_ARGS)

lint:
	cargo clippy $(LINT_ARGS)

sec-dependencies:
	cargo install cargo-audit

sec:
	cargo audit
