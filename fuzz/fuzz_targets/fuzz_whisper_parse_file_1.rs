#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate whisper;

use whisper::parser::whisper_parse_file;

fuzz_target!(|data: &[u8]| {
    let _r = whisper_parse_file(data);
});