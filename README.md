# About
This project attempts to implement a part of the [OAuth 2.0](https://datatracker.ietf.org/doc/html/rfc6749) specification in Rust with a modular approach so that it can be implemented in other's backend application.
It utilizes the [Axum](https://github.com/tokio-rs/axum) framework to perform authorization tasks asynchronously.

## Design choices
Limited to **code** response mode for now.

## Contributors
Abdur Rahman Goraya - Lead developer and maintainer

### TODO
1. Access and Refresh Token exchange endpoints
2. Refactor Cache to async
3. Refactor `cache.rs` to use struct with impl instead of functional programming
4. Refactor `database.rs` to use struct with impl instead of functional programming
5. Refactor or delete or blow out of existence `storage.rs`
6. Refactor project tree
7. Optimize code
