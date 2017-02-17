# Logstash Format

A small library which configures [env_logger](http://rust-lang-nursery.github.io/log/env_logger/) to write in logstash format.

```rust
#[macro_use] extern crate log;
#[macro_use] extern crate serde_json;
extern crate logstash_format;

fn main() {
    logstash_format::new_builder(json!({
        "app": "foo",
    })).init().unwrap();

    error!("hello world");
}
```
