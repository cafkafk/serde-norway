#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() <= 10240 {
        _ = serde_norway::from_slice::<serde_norway::Value>(data);
    }
});
