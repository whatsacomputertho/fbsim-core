BUILD_ARGS ?= --release
RELEASE_ARGS ?= --dry-run
LINT_ARGS ?= --all-targets --all-features -- -D warnings

build-dependencies:
	cargo install wasm-pack

build:
	cargo build $(BUILD_ARGS)
	npm run build

test:
	cargo test

release:
	cargo publish $(RELEASE_ARGS)
	npm publish

lint:
	cargo clippy $(LINT_ARGS)

sec-dependencies:
	cargo install cargo-audit

sec:
	cargo audit

docs-js:
	npm run docs
