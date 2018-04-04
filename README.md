# Rust musl libc Rocket Memory Leak Reproduction

Steps to Reproduce:

1. Run `./run-musl-docker.bash` to build the app as a musl libc static binary and run it
2. `wget http://localhost:8000/25000000` to allocate a 25MB string and return it
3. Observe that the 25MB allocated by each request isn't freed at the end of the request.
4. Try building it + running it normally via `cargo run --release`, repeat #2, and observe that memory IS freed as it should be.
