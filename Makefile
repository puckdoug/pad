default: pad lpad rpad

target/debug/pad:
	cargo build

target/debug/lpad: target/debug/pad
	(cd target/debug && ln pad lpad)

target/debug/rpad: target/debug/pad
	(cd target/debug && ln pad rpad)

pad: target/debug/pad
lpad: target/debug/lpad
rpad: target/debug/rpad

target/release/pad:
	cargo build --release

release: target/release/pad
	(cd target/release && ln pad lpad)
	(cd target/release && ln pad rpad)

clean:
	cargo clean
