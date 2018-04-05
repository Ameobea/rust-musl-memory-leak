# Rust musl libc Rocket Memory Leak Reproduction

Steps to Reproduce:

1. `docker build -t memleak`
2. `docker run -it -p 4545:4545 memleak`
