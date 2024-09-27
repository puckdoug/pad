SRC = $(wildcard src/*.rs)

default: pad lpad rpad

target/debug/pad: $(SRC)
	cargo build

target/debug/lpad: target/debug/pad
	(cd target/debug && rm -f lpad && ln pad lpad)

target/debug/rpad: target/debug/pad
	(cd target/debug && rm -f rpad && ln pad rpad)

pad: target/debug/pad
lpad: target/debug/lpad
rpad: target/debug/rpad

target/release/pad:
	cargo build --release

release: target/release/pad
	(cd target/release && rm -f lpad && ln pad lpad)
	(cd target/release && rm -f rpad && ln pad rpad)

clean:
	cargo clean

test:
	cargo test

cov:
	cargo tarpaulin

testwatch:
	cargo watch -x test

install: release
	cargo install --path .

doc: $(SRC)
	cargo doc

readdoc:
	cargo doc --open

deb:
	cargo deb

deb-install:
	cargo deb --install
