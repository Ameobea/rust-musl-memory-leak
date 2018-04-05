alias rust-musl-builder='docker run --rm -it -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder'
rust-musl-builder "rustup default nightly-2018-04-04 && \
    rust-musl-builder cargo build --release --target x86_64-unknown-linux-musl"

./target/x86_64-unknown-linux-musl/release/rocket-mem-leak
