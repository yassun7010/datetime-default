//! # DateTime NewType set for default time
//!
//! [![Latest Version](https://img.shields.io/crates/v/datetime-default.svg?color=green&style=flat-square)](https://crates.io/crates/datetime-default)
//! [![GitHub license](https://badgen.net/github/license/Naereen/Strapdown.js?style=flat-square)](https://github.com/Naereen/StrapDown.js/blob/master/LICENSE)
//!
//! When you want to use `#[derive(Default)]` to a struct, you may want the current time to be the default for `DateTime`.
//! For this purpose, we have provided NewType that overrides the default value.
//!
//! ## Examples
//! ### DateTimeDefaultNow
//! DateTime with current time as default
//!
//! ```should_panic
//! use chrono::{Local, TimeZone};
//! use datetime_default::DateTimeDefaultNow;
//!
//! assert_eq!(
//!     DateTimeDefaultNow::<Local>::default(),
//!     Local.datetime_from_str("2022/10/10 23:40:11.695164300", "%Y/%m/%d %H:%M:%S%.9f").unwrap()
//! );
//! ```
//!
//! ### DateTimeDefaultUnix
//! DateTime with UNIX epoch as default
//!
//! ```
//! use chrono::{Utc, TimeZone};
//! use datetime_default::DateTimeDefaultUnix;
//!
//! assert_eq!(
//!     DateTimeDefaultUnix::<Utc>::default(),
//!     Utc.datetime_from_str("1970/1/1 00:00:00", "%Y/%m/%d %H:%M:%S").unwrap()
//! );
//! ```
//!
//! ### Default Datetime with TimeZone
//!
//! ```
//! use chrono::{DateTime, FixedOffset, TimeZone};
//! use datetime_default::DateTimeDefaultUnix;
//!
//! assert_eq!(
//!     DateTimeDefaultUnix::<FixedOffset, 9>::default(),
//!     DateTime::parse_from_rfc3339("1970-01-01T09:00:00+09:00").unwrap()
//! );
//! ```
//!
mod datetime_default;
mod datetime_default_now;
mod datetime_default_unix;

pub use datetime_default_now::DateTimeDefaultNow;
pub use datetime_default_unix::DateTimeDefaultUnix;
