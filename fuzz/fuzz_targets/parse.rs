#![no_main]

use libfuzzer_sys::fuzz_target;
use saphyr_parser::{Event, EventReceiver, Parser};

struct Sink {}

impl EventReceiver for Sink {
    fn on_event(&mut self, _ev: Event) {}
}

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let mut parser = Parser::new_from_str(s);
        let mut sink = Sink {};
        let _ = parser.load(&mut sink, true);
    }
});
