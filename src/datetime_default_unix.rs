use std::ops::Deref;

use chrono::{DateTime, Duration, FixedOffset, Local, TimeZone, Utc};

use crate::const_assert::AssertOffsetHours;

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
pub struct DateTimeDefaultUnix<Tz, const OFFSET_HOURS: i32 = 0>(DateTime<Tz>)
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy;

impl<const OFFSET_HOURS: i32> Default for DateTimeDefaultUnix<FixedOffset, OFFSET_HOURS> {
    #[allow(path_statements)]
    #[allow(clippy::no_effect)]
    fn default() -> Self {
        AssertOffsetHours::<-24, OFFSET_HOURS, 24>::OK;

        Self(
            DateTimeDefaultUnix::<Utc>::default()
                .with_timezone(&FixedOffset::east_opt(OFFSET_HOURS * 3600).unwrap()),
        )
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

impl<Tz, const OFFSET_HOURS: i32> Deref for DateTimeDefaultUnix<Tz, OFFSET_HOURS>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    type Target = DateTime<Tz>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Tz, const OFFSET_HOURS: i32> From<DateTime<Tz>> for DateTimeDefaultUnix<Tz, OFFSET_HOURS>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    fn from(datetime: DateTime<Tz>) -> Self {
        Self(datetime)
    }
}

impl<const OFFSET_HOURS: i32> std::str::FromStr for DateTimeDefaultUnix<FixedOffset, OFFSET_HOURS> {
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> chrono::ParseResult<DateTimeDefaultUnix<FixedOffset, OFFSET_HOURS>> {
        DateTime::<FixedOffset>::from_str(s).map(DateTimeDefaultUnix::from)
    }
}

impl<const OFFSET_HOURS: i32> std::str::FromStr for DateTimeDefaultUnix<Local, OFFSET_HOURS> {
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> chrono::ParseResult<DateTimeDefaultUnix<Local, OFFSET_HOURS>> {
        DateTime::<Local>::from_str(s).map(DateTimeDefaultUnix::from)
    }
}

impl<const OFFSET_HOURS: i32> std::str::FromStr for DateTimeDefaultUnix<Utc, OFFSET_HOURS> {
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> chrono::ParseResult<DateTimeDefaultUnix<Utc, OFFSET_HOURS>> {
        DateTime::<Utc>::from_str(s).map(DateTimeDefaultUnix::from)
    }
}

impl<Tz, const OFFSET_HOURS: i32> std::cmp::PartialEq<DateTime<Tz>>
    for DateTimeDefaultUnix<Tz, OFFSET_HOURS>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    fn eq(&self, other: &DateTime<Tz>) -> bool {
        self.0.eq(other)
    }
}

impl<Tz, const OFFSET_HOURS: i32> std::cmp::PartialEq<DateTimeDefaultUnix<Tz, OFFSET_HOURS>>
    for DateTime<Tz>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    fn eq(&self, other: &DateTimeDefaultUnix<Tz, OFFSET_HOURS>) -> bool {
        self.eq(&other.0)
    }
}

impl<Tz, const OFFSET_HOURS: i32> std::cmp::PartialOrd<DateTime<Tz>>
    for DateTimeDefaultUnix<Tz, OFFSET_HOURS>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    fn partial_cmp(&self, other: &DateTime<Tz>) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl<Tz, const OFFSET_HOURS: i32> std::cmp::PartialOrd<DateTimeDefaultUnix<Tz, OFFSET_HOURS>>
    for DateTime<Tz>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    fn partial_cmp(
        &self,
        other: &DateTimeDefaultUnix<Tz, OFFSET_HOURS>,
    ) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&other.0)
    }
}

impl<Tz, const OFFSET_HOURS: i32> std::ops::Add<Duration> for DateTimeDefaultUnix<Tz, OFFSET_HOURS>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    type Output = DateTimeDefaultUnix<Tz, OFFSET_HOURS>;

    #[inline]
    fn add(self, rhs: Duration) -> Self::Output {
        DateTimeDefaultUnix(self.0.add(rhs))
    }
}

impl<Tz, const OFFSET_HOURS: i32> std::ops::Add<FixedOffset>
    for DateTimeDefaultUnix<Tz, OFFSET_HOURS>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    type Output = DateTimeDefaultUnix<Tz, OFFSET_HOURS>;

    #[inline]
    fn add(self, rhs: FixedOffset) -> Self::Output {
        DateTimeDefaultUnix(self.0.add(rhs))
    }
}

impl<Tz, const OFFSET_HOURS: i32> std::ops::AddAssign<Duration>
    for DateTimeDefaultUnix<Tz, OFFSET_HOURS>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    #[inline]
    fn add_assign(&mut self, rhs: Duration) {
        self.0.add_assign(rhs);
    }
}

impl<Tz, const OFFSET_HOURS: i32> std::ops::Sub<DateTimeDefaultUnix<Tz, OFFSET_HOURS>>
    for DateTimeDefaultUnix<Tz, OFFSET_HOURS>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    type Output = Duration;

    #[inline]
    fn sub(self, rhs: DateTimeDefaultUnix<Tz, OFFSET_HOURS>) -> Duration {
        self.0.sub(rhs.0)
    }
}

impl<Tz, const OFFSET_HOURS: i32> std::ops::Sub<DateTime<Tz>>
    for DateTimeDefaultUnix<Tz, OFFSET_HOURS>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    type Output = Duration;

    #[inline]
    fn sub(self, rhs: DateTime<Tz>) -> Duration {
        self.0.sub(rhs)
    }
}

impl<Tz, const OFFSET_HOURS: i32> std::ops::Sub<Duration> for DateTimeDefaultUnix<Tz, OFFSET_HOURS>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    type Output = DateTimeDefaultUnix<Tz, OFFSET_HOURS>;

    #[inline]
    fn sub(self, rhs: Duration) -> Self::Output {
        DateTimeDefaultUnix(self.0.sub(rhs))
    }
}

impl<Tz, const OFFSET_HOURS: i32> std::ops::Sub<FixedOffset>
    for DateTimeDefaultUnix<Tz, OFFSET_HOURS>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    type Output = DateTimeDefaultUnix<Tz, OFFSET_HOURS>;

    #[inline]
    fn sub(self, rhs: FixedOffset) -> Self::Output {
        DateTimeDefaultUnix(self.0.sub(rhs))
    }
}

impl<Tz, const OFFSET_HOURS: i32> std::ops::SubAssign<Duration>
    for DateTimeDefaultUnix<Tz, OFFSET_HOURS>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    #[inline]
    fn sub_assign(&mut self, rhs: Duration) {
        self.0.sub_assign(rhs);
    }
}

impl<Tz, const OFFSET_HOURS: i32> chrono::DurationRound for DateTimeDefaultUnix<Tz, OFFSET_HOURS>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    type Err = chrono::RoundingError;

    fn duration_round(self, duration: Duration) -> Result<Self, Self::Err> {
        self.0
            .duration_round(duration)
            .map(DateTimeDefaultUnix::from)
    }

    fn duration_trunc(self, duration: Duration) -> Result<Self, Self::Err> {
        self.0
            .duration_trunc(duration)
            .map(DateTimeDefaultUnix::from)
    }
}

impl<Tz, const OFFSET_HOURS: i32> chrono::Datelike for DateTimeDefaultUnix<Tz, OFFSET_HOURS>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    #[inline]
    fn year(&self) -> i32 {
        self.0.year()
    }
    #[inline]
    fn month(&self) -> u32 {
        self.0.month()
    }
    #[inline]
    fn month0(&self) -> u32 {
        self.0.month0()
    }
    #[inline]
    fn day(&self) -> u32 {
        self.0.day()
    }
    #[inline]
    fn day0(&self) -> u32 {
        self.0.day0()
    }
    #[inline]
    fn ordinal(&self) -> u32 {
        self.0.ordinal()
    }
    #[inline]
    fn ordinal0(&self) -> u32 {
        self.0.ordinal0()
    }
    #[inline]
    fn weekday(&self) -> chrono::Weekday {
        self.0.weekday()
    }
    #[inline]
    fn iso_week(&self) -> chrono::IsoWeek {
        self.0.iso_week()
    }

    #[inline]
    fn with_year(&self, year: i32) -> Option<DateTimeDefaultUnix<Tz, OFFSET_HOURS>> {
        self.0.with_year(year).map(DateTimeDefaultUnix::from)
    }

    #[inline]
    fn with_month(&self, month: u32) -> Option<DateTimeDefaultUnix<Tz, OFFSET_HOURS>> {
        self.0.with_month(month).map(DateTimeDefaultUnix::from)
    }

    #[inline]
    fn with_month0(&self, month0: u32) -> Option<DateTimeDefaultUnix<Tz, OFFSET_HOURS>> {
        self.0.with_month0(month0).map(DateTimeDefaultUnix::from)
    }

    #[inline]
    fn with_day(&self, day: u32) -> Option<DateTimeDefaultUnix<Tz, OFFSET_HOURS>> {
        self.0.with_day(day).map(DateTimeDefaultUnix::from)
    }

    #[inline]
    fn with_day0(&self, day0: u32) -> Option<DateTimeDefaultUnix<Tz, OFFSET_HOURS>> {
        self.0.with_day0(day0).map(DateTimeDefaultUnix::from)
    }

    #[inline]
    fn with_ordinal(&self, ordinal: u32) -> Option<DateTimeDefaultUnix<Tz, OFFSET_HOURS>> {
        self.0.with_ordinal(ordinal).map(DateTimeDefaultUnix::from)
    }

    #[inline]
    fn with_ordinal0(&self, ordinal0: u32) -> Option<DateTimeDefaultUnix<Tz, OFFSET_HOURS>> {
        self.0
            .with_ordinal0(ordinal0)
            .map(DateTimeDefaultUnix::from)
    }
}

impl<Tz, const OFFSET_HOURS: i32> chrono::Timelike for DateTimeDefaultUnix<Tz, OFFSET_HOURS>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    #[inline]
    fn hour(&self) -> u32 {
        self.0.hour()
    }
    #[inline]
    fn minute(&self) -> u32 {
        self.0.minute()
    }
    #[inline]
    fn second(&self) -> u32 {
        self.0.second()
    }
    #[inline]
    fn nanosecond(&self) -> u32 {
        self.0.nanosecond()
    }

    #[inline]
    fn with_hour(&self, hour: u32) -> Option<DateTimeDefaultUnix<Tz, OFFSET_HOURS>> {
        self.0.with_hour(hour).map(DateTimeDefaultUnix::from)
    }

    #[inline]
    fn with_minute(&self, min: u32) -> Option<DateTimeDefaultUnix<Tz, OFFSET_HOURS>> {
        self.0.with_minute(min).map(DateTimeDefaultUnix::from)
    }

    #[inline]
    fn with_second(&self, sec: u32) -> Option<DateTimeDefaultUnix<Tz, OFFSET_HOURS>> {
        self.0.with_second(sec).map(DateTimeDefaultUnix::from)
    }

    #[inline]
    fn with_nanosecond(&self, nano: u32) -> Option<DateTimeDefaultUnix<Tz, OFFSET_HOURS>> {
        self.0.with_nanosecond(nano).map(DateTimeDefaultUnix::from)
    }
}

impl<Tz, const OFFSET_HOURS: i32> std::fmt::Display for DateTimeDefaultUnix<Tz, OFFSET_HOURS>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
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
    fn raise_max_offset_hours() {
        DateTimeDefaultUnix::<FixedOffset, 23>::default();
    }

    #[test]
    fn raise_min_offset_hours() {
        DateTimeDefaultUnix::<FixedOffset, -23>::default();
    }

    #[test]
    fn fixed_offset() {
        let datetime = DateTimeDefaultUnix::<FixedOffset, 9>::default();

        assert_eq!(datetime.to_rfc3339(), "1970-01-01T09:00:00+09:00");
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
