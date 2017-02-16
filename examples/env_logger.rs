#[macro_use]
extern crate log;
extern crate logstash_format;

fn main() {
    logstash_format::new_builder("foo").init().unwrap();

    error!("hello world");
    error!(r###"{{"foo": "bar"}}"###);
    println!("Main");
}
