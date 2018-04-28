format:
	cargo fmt --all -- --write-mode=diff
.PHONY: format

fmt: | format
.PHONY: fmt

lint:
	cargo rustc --features clippy -- -Z no-trans -Z extra-plugins=clippy
.PHONY: lint

test:
	cd src/sprinkle && cargo test
.PHONY: test

clean:
	cd src/sprinkle && cargo clean
	cargo clean
.PHONY: clean
