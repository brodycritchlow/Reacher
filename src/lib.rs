#![warn(clippy::nursery, clippy::pedantic)]
#![allow(
clippy::cast_possible_truncation,
clippy::cast_sign_loss,
clippy::cast_precision_loss,
clippy::module_name_repetitions,
clippy::unused_self,
clippy::return_self_not_must_use,
clippy::must_use_candidate
)]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

mod filter;
mod search;
mod utils;

pub use search::SearchBuilder;
pub use filter::{FileSize, FilterExt, FilterFn};

// Custom filter function exports:

pub use ignore::DirEntry;
pub use search::Search;
pub use utils::similarity_sort;