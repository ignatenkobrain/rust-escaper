extern crate entities;

pub use decode::*;
pub use encode::*;

mod decode;
mod encode;
mod io_support;

pub static MINIMAL_ENTITIES: [(char, &str); 5] = [
    ('"', "&quot;"),
    ('&', "&amp;"),
    ('\'', "&#x27;"),
    ('<', "&lt;"),
    ('>', "&gt;"),
];
