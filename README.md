# real_ua

[![License](https://img.shields.io/badge/license-GNU_GPLv3-blue.svg)](https://choosealicense.com/licenses/gpl-3.0/)
[![Cargo](https://img.shields.io/crates/v/real_ua.svg)](https://crates.io/crates/real_ua)
[![Documentation](https://docs.rs/real_ua/badge.svg)](https://docs.rs/real_ua)

static real user-agent automatically updated

```rust
fn main() {
    let user_agent: &'static str = real_ua::get_ua();
    println!("{}", user_agent);
}
```

It depends on [fastrand](https://github.com/smol-rs/fastrand) to select randomly a user-agent from a static list.

## TODO

* update user agents using GitHub workflow
