//! A Rust implementation of DEFLATE algorithm and related formats (ZLIB, GZIP).
#![warn(missing_docs)]
extern crate byteorder;

pub use finish::Finish;

macro_rules! invalid_data_error {
    ($fmt:expr) => { invalid_data_error!("{}", $fmt) };
    ($fmt:expr, $($arg:tt)*) => {
        ::std::io::Error::new(::std::io::ErrorKind::InvalidData, format!($fmt, $($arg)*))
    }
}

macro_rules! finish_try {
    ($e:expr) => {
        match $e.unwrap() {
            (inner, None) => inner,
            (inner, error) => return ::finish::Finish::new(inner, error)
        }
    }
}

pub mod lz77;
pub mod zlib;
pub mod gzip;
pub mod deflate;

mod bit;
mod finish;
mod huffman;
mod checksum;
