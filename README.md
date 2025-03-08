# zero2prod
Zero to production in Rust

I'm going by the book Zero to Production in Rust and trying to get my Rust chops up. Here's a list of good commands to know when developing this project.

## Cargo

cargo watch -x check # run cargo every time a change is made
cargo watch -x check -x test -x run # an example of command chaining to build, test, and run the application


## Linting 
cargo clippy
cargo clippy -- -D warnings #fail if there's warnings

### when getting a bunk suggestion ignore warnings with #[allow(clippy::lint_name)]
### or in a file outside of code 
### clippy.toml -> #![allow(clippy::lint_name)]

## Code coverage
cargo tarpaulin --ignore-tests

## Security 
cargo-audit # search the Rust Secure Code working group Advisory Database for vulnerable packages

