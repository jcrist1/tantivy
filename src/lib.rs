#![allow(
    clippy::len_without_is_empty,
    clippy::derive_partial_eq_without_eq,
    clippy::module_inception,
    clippy::needless_range_loop,
    clippy::bool_assert_comparison
)]

pub(crate) use tantivy_common as common;
pub(crate) use tantivy_tokenizer_api as tokenizer_api;

pub mod error;
pub mod tokenizer;
pub use error::*;

type Result<T> = std::result::Result<T, TantivyError>;
