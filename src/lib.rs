extern crate log;
extern crate env_logger;
extern crate chrono;
extern crate rustc_serialize;

use std::env;
use log::{LogRecord, LogLevelFilter};
use env_logger::LogBuilder;
use rustc_serialize::json::encode;
use chrono::Local;

pub fn new_builder(app: &'static str) -> LogBuilder {
    let mut builder = LogBuilder::new();

    let format = move |record: &LogRecord| {
        let message = record.args().to_string();
        let string = vec![
            format!("\"@timestamp\":\"{:?}\"", Local::now()),
            format!("\"@version\":\"{}\"", "1"),
            format!("\"message\":{}", encode(&message).unwrap()),
            format!("\"level\":\"{}\"", record.level()),
            format!("\"app\":\"{}\"", app),
        ].join(",");

        format!("{{{}}}", string)
    };

    builder.format(format).filter(None, LogLevelFilter::Info);

    if env::var("RUST_LOG").is_ok() {
        builder.parse(&env::var("RUST_LOG").unwrap());
    }

    builder
}
