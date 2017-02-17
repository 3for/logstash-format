extern crate log;
extern crate env_logger;
#[macro_use]
extern crate serde_json;
extern crate time;

use std::env;
use log::{LogRecord, LogLevelFilter};
use env_logger::LogBuilder;
use serde_json::Value;

pub fn new_builder(attrs: Value) -> LogBuilder {
    if !attrs.is_object() {
        panic!("Error: attrs must be an object.");
    }

    let format = move |record: &LogRecord| {
        let mut object = json!({
            "@timestamp": time::now().rfc3339().to_string(),
            "@version": "1",
            "message": record.args().to_string(),
            "level": record.level().to_string(),
        });

        if let Some(obj) = object.as_object_mut() {
            for (k, v) in attrs.as_object().unwrap() {
                obj.insert(k.to_owned(), v.to_owned());
            }
        }

        serde_json::to_string(&object).unwrap()
    };

    let mut builder = LogBuilder::new();
    builder.format(format).filter(None, LogLevelFilter::Info);

    if env::var("RUST_LOG").is_ok() {
        builder.parse(&env::var("RUST_LOG").unwrap());
    }

    builder
}
