#![no_std]
#![warn(missing_docs, unused_lifetimes)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! Bincode is a crate for encoding and decoding using a tiny binary
//! serialization strategy.  Using it, you can easily go from having
//! an object in memory, quickly serialize it to bytes, and then
//! deserialize it back just as fast!
//!
//! # Serde
//!
//! Starting from bincode 2, serde is now an optional dependency. If you want to use serde, please enable the `serde` feature. See [Features](#features) for more information.
//!
//! # Features
//!
//! |Name  |Default?|Supported types for Encode/Decode|Enabled methods                                                  |Other|
//! |------|--------|-----------------------------------------|-----------------------------------------------------------------|-----|
//! |std   | Yes    ||`decode_from_reader` and `encode_into_writer`|
//! |alloc | Yes    |All common containers in alloc, like `Vec`, `String`, `Box`|`encode_to_vec`|
//! |atomic| Yes    |All `Atomic*` integer types, e.g. `AtomicUsize`, and `AtomicBool`||
//! |derive| Yes    |||Enables the `Encode` and `Decode` derive macro|
//! |serde | No     |TODO|TODO|TODO|

#![doc(html_root_url = "https://docs.rs/bincode/2.0.0-alpha.0")]
#![crate_name = "bincode"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(any(feature = "std", test))]
extern crate std;

mod features;
pub(crate) mod varint;

pub use features::*;

pub mod config;
pub mod de;
pub mod enc;
pub mod error;

use config::Config;

/// Encode the given value into the given slice. Returns the amount of bytes that have been written.
///
/// See the [config] module for more information on configurations.
///
/// [config]: config/index.html
pub fn encode_into_slice<E: enc::Encode, C: Config>(
    val: E,
    dst: &mut [u8],
    config: C,
) -> Result<usize, error::EncodeError> {
    let writer = enc::write::SliceWriter::new(dst);
    let mut encoder = enc::EncoderImpl::<_, C>::new(writer, config);
    val.encode(&mut encoder)?;
    Ok(encoder.into_writer().bytes_written())
}

/// Attempt to decode a given type `D` from the given slice.
///
/// See the [config] module for more information on configurations.
///
/// [config]: config/index.html
pub fn decode_from_slice<'a, D: de::BorrowDecode<'a>, C: Config>(
    src: &'a [u8],
    _config: C,
) -> Result<D, error::DecodeError> {
    let reader = de::read::SliceReader::new(src);
    let mut decoder = de::DecoderImpl::<_, C>::new(reader, _config);
    D::borrow_decode(&mut decoder)
}

// TODO: Currently our doctests fail when trying to include the specs because the specs depend on `derive` and `alloc`.
// But we want to have the specs in the docs always
#[cfg(all(feature = "alloc", feature = "derive"))]
pub mod spec {
    #![doc = include_str!("../docs/spec.md")]
}
