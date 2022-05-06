# Deployer

[![deployer crate](https://img.shields.io/crates/v/deployer.svg)](https://crates.io/crates/deployer)
[![test](https://github.com/samirdjelal/deployer/workflows/test/badge.svg)](https://github.com/samirdjelal/deployer/actions)
[![build](https://github.com/samirdjelal/deployer/workflows/build/badge.svg)](https://github.com/samirdjelal/deployer/actions)
[![issues](https://img.shields.io/github/issues/samirdjelal/deployer?color=%23ffc107)](https://github.com/samirdjelal/deployer/issues)
[![Downloads](https://img.shields.io/crates/d/deployer)](https://crates.io/crates/deployer)
[![MIT License](https://img.shields.io/crates/l/deployer)](LICENSE)
[![deployer documentation](https://img.shields.io/docsrs/deployer)](https://docs.rs/deployer)
[![dependency status](https://deps.rs/repo/github/samirdjelal/deployer/status.svg)](https://deps.rs/repo/github/samirdjelal/deployer)

A simple Laravel deployer for your projects.

## Example

### Using the binary

```bash
💲 cargo install deployer
💲 deployer config.yml
```

### Using the crate
Add the following dependency to the Cargo.toml file:

```toml
[dependencies]
deployer = "0.0.1"
```

And then get started in your `main.rs`:

```rust
use deployer::Deployer;

fn main() {
	
	let config = "config.yml";
	
	let mut deployer = Deployer::new();
	deployer.configure(config);
	deployer.deploy();
	
}
```

```bash
# Dev
💲 cargo run -- config.yml

# Build
💲 cargo build
💲 target/debug/deployer config.yml

# Test
💲 cargo test
```

---

## License

This project is licensed under the [MIT license](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in `deployer` by you, shall be licensed as MIT, without any additional terms or conditions.
