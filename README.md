# real_ua

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
