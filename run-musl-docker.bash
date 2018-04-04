alias rust-musl-builder='docker run --rm -it -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder'
rust-musl-builder rustup default nightly-2018-03-15 && rustup target add x86_64-unknown-linux-musl && cargo build --release --target x86_64-unknown-linux-musl
./target/x86_64-unknown-linux-musl/release/rocket-mem-leak
