#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate crypto_hashes;

use crypto_hashes::digest::Digest;

fuzz_target!(|data| {
    let mut hasher = crypto_hashes::ripemd160::Ripemd160::default();
    hasher.input(data);
    hasher.result();
});

