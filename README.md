# Logstash Format

将env_logger日志信息组装成logstashformat格式。

```
git clone https://github.com/3for/logstash-format.git
cd logstash-format
cargo run --example example
```

运行的结果为：
```
{"@timestamp":"2019-05-08T17:54:55+08:00","@version":"1","app":"foo","level":"ERROR","loglevel":"ERROR","message":"hello world"}
{"@timestamp":"2019-05-08T17:54:55+08:00","@version":"1","app":"foo","level":"ERROR","loglevel":"ERROR","message":"{\"foo\": \"bar\"}"}
Main
```

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
