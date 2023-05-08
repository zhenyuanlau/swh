.PHONY: build
build:
	cargo build --release

.PHONY: install
install: build
	cp target/release/swh ~/.cargo/bin/swh
	chmod +x ~/.cargo/bin/swh

.PHONY: code
code:
	cargo clippy
	cargo fmt

# https://doc.rust-lang.org/cargo/reference/publishing.html
.PHONY: publish
publish:
	cargo publish --dry-run