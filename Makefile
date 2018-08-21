# cargo fmt --all -- --write-mode=diff
format:
	cargo fmt --all -- --check
.PHONY: format

fmt: | format
.PHONY: fmt

# cargo rustc --features clippy -- -Z no-trans -Z extra-plugins=clippy
lint:
	cargo clippy --all-targets
.PHONY: lint

test:
	cd src/sprinkle && cargo test
	cd src/coolify && cargo test
	cd src/domainify && cargo test
.PHONY: test

clean:
	cd src/sprinkle && cargo clean
	cd src/coolify && cargo clean
	cd src/domainify && cargo clean
	cargo clean
.PHONY: clean
