#[macro_use] extern crate log;
#[macro_use] extern crate serde_json;
extern crate logstash_format;

fn main() {
    logstash_format::new_builder(|default| json!({
        "app": "foo",
        "loglevel": default.get("level").unwrap(),
    })).init().unwrap();

    error!("hello world");
    error!(r###"{{"foo": "bar"}}"###);
    println!("Main");
}
