use std::ops::Deref;

use chrono::{DateTime, FixedOffset, Local, TimeZone, Utc};

/// # DateTime with UNIX epoch as default.
///
/// ```
/// use chrono::{Utc, TimeZone};
/// use datetime_default::DateTimeDefaultUnix;
///
/// assert_eq!(
///     DateTimeDefaultUnix::<Utc>::default(),
///     Utc.datetime_from_str("1970/1/1 00:00:00", "%Y/%m/%d %H:%M:%S").unwrap()
/// );
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DateTimeDefaultUnix<Tz, const N: i32 = 0>(DateTime<Tz>)
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy;

impl<const N: i32> Default for DateTimeDefaultUnix<FixedOffset, N> {
    fn default() -> Self {
        Self(DateTimeDefaultUnix::<Utc>::default().with_timezone(&FixedOffset::east(N * 3600)))
    }
}

impl Default for DateTimeDefaultUnix<Local, 0> {
    fn default() -> Self {
        Self(DateTime::<Local>::default())
    }
}

impl Default for DateTimeDefaultUnix<Utc, 0> {
    fn default() -> Self {
        Self(DateTime::<Utc>::default())
    }
}

impl<Tz, const N: i32> Deref for DateTimeDefaultUnix<Tz, N>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    type Target = DateTime<Tz>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Tz, const N: i32> From<DateTime<Tz>> for DateTimeDefaultUnix<Tz, N>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    fn from(datetime: DateTime<Tz>) -> Self {
        Self(datetime)
    }
}

impl<Tz, const N: i32> std::cmp::PartialEq<DateTime<Tz>> for DateTimeDefaultUnix<Tz, N>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    fn eq(&self, other: &DateTime<Tz>) -> bool {
        self.0.eq(other)
    }
}

impl<Tz, const N: i32> std::cmp::PartialEq<DateTimeDefaultUnix<Tz, N>> for DateTime<Tz>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    fn eq(&self, other: &DateTimeDefaultUnix<Tz, N>) -> bool {
        self.eq(&other.0)
    }
}

impl<Tz, const N: i32> std::cmp::PartialOrd<DateTime<Tz>> for DateTimeDefaultUnix<Tz, N>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    fn partial_cmp(&self, other: &DateTime<Tz>) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl<Tz, const N: i32> std::cmp::PartialOrd<DateTimeDefaultUnix<Tz, N>> for DateTime<Tz>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    fn partial_cmp(&self, other: &DateTimeDefaultUnix<Tz, N>) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&other.0)
    }
}

#[cfg(test)]
mod tests {

    use chrono::{DateTime, FixedOffset, Local, Utc};

    use crate::DateTimeDefaultUnix;

    #[derive(Default)]
    struct Test {
        updated_at: DateTimeDefaultUnix<Utc>,
    }

    fn print_datetime(datetime: &DateTime<Utc>) {
        println!("{datetime}")
    }

    #[test]
    fn default_derive_struct() {
        Test::default();
    }

    #[test]
    fn use_default_derive_struct() {
        let test = Test::default();

        print_datetime(&test.updated_at)
    }

    #[test]
    fn use_default_datetime() {
        let datetime = DateTimeDefaultUnix::<Utc>::default();

        print_datetime(&datetime)
    }

    #[test]
    fn fixed_offset() {
        let datetime = DateTimeDefaultUnix::<FixedOffset, 9>::default();
        println!("{datetime:?}");

        // assert!(false);
    }

    #[test]
    fn use_datetime_method() {
        let datetime = DateTimeDefaultUnix::<Utc>::default();

        datetime.date();
    }

    #[test]
    fn convert_from() {
        let now = DateTime::<Local>::default();
        let datetime = DateTimeDefaultUnix::<Local>::from(now);

        assert_eq!(datetime, now)
    }

    #[test]
    fn convert_into() {
        let now = DateTime::<Local>::default();
        let datetime: DateTimeDefaultUnix<Local> = now.into();

        assert_eq!(datetime, now)
    }

    #[test]
    fn equation1() {
        let now = DateTime::<Local>::default();
        let datetime: DateTimeDefaultUnix<Local> = now.into();
        assert!(datetime == now)
    }

    #[test]
    fn equation2() {
        let now = DateTime::<Local>::default();
        let datetime: DateTimeDefaultUnix<Local> = now.into();
        assert!(now == datetime)
    }

    #[test]
    fn comparing1() {
        assert!(DateTimeDefaultUnix::<Local>::default() <= DateTime::<Local>::default())
    }

    #[test]
    fn comparing2() {
        assert!(DateTime::<Local>::default() <= DateTimeDefaultUnix::<Local>::default())
    }
}
