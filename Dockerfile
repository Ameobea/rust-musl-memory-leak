FROM ekidd/rust-musl-builder

RUN ~/.cargo/bin/rustup default nightly-2018-04-05
RUN ~/.cargo/bin/rustup target add x86_64-unknown-linux-musl
RUN ~/.cargo/bin/rustup update

ADD . /home/rust/src
RUN sudo chown -R rust:rust /home/rust

RUN cargo build --target x86_64-unknown-linux-musl

CMD /home/rust/src/target/x86_64-unknown-linux-musl/debug/rocket-mem-leak
