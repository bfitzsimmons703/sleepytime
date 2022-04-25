## [Sleepytime.rs](https://sleepytime-rs.onrender.com/sleep/100)
### A simple API to simulate slow network requests.


I often need something like this when developing personal projects. And it's a pain to have to copy/paste code to put the current thread to sleep for X milliseconds.

Plus, it was a fun opportunity to learn some Rust.

---
## How to build locally
If you don't want to use the public endpoint linked above, you can build and use sleepytime locally.

### With Rust:
Make sure you have rust and cargo installed. This uses rust 1.60.
CD into the directoy and run `cargo run`

### With Docker:
Run `docker-compose up --build`

---
## How to use locally
Send a get request to localhost:8080/sleep/{ms} where {ms} is the number of milliseconds you want the request to hang for