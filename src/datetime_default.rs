//! # DateTime with user specified default.
//!
//! This document is written as a reservation for the future.
//! `DateTimeDefault` will be implemented when `const &'static str` become available as generic parameters.
//!
//! ```compile_fail
//! use chrono::{Utc, TimeZone};
//!
//! #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
//! pub struct DateTimeDefault<const DEFAULT: &'static str, Tz, const OFFSET_HOURS: i32 = 0>(
//!     DateTime<Tz>,
//! )
//! where
//!     Tz: TimeZone,
//!     <Tz as TimeZone>::Offset: Copy;
//!
//! // Usage
//! assert_eq!(
//!     DateTimeDefault::<"2000-01-01T00:00:00", Utc>::default(),
//!     Utc.datetime_from_str("2000/1/1 00:00:00", "%Y/%m/%d %H:%M:%S").unwrap()
//! );
//! ```
//!
