use std::ops::Deref;

use chrono::{DateTime, Duration, FixedOffset, Local, TimeZone, Utc};

use crate::const_assert::AssertOffsetHours;

#[cfg(test)]
const NOW: &str = "2022/10/10 23:40:11.695164300";

/// # DateTime with current time as default.
///
/// ```should_panic
/// use chrono::{Local, TimeZone};
/// use datetime_default::DateTimeDefaultNow;
///
/// assert_eq!(
///     DateTimeDefaultNow::<Local>::default(),
///     Local.datetime_from_str("2022/10/10 23:40:11.695164300", "%Y/%m/%d %H:%M:%S%.9f").unwrap()
/// );
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DateTimeDefaultNow<Tz, const OFFSET_HOURS: i32 = 0>(DateTime<Tz>)
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy;

impl<const OFFSET_HOURS: i32> Default for DateTimeDefaultNow<FixedOffset, OFFSET_HOURS> {
    #[allow(path_statements)]
    #[allow(clippy::no_effect)]
    fn default() -> Self {
        AssertOffsetHours::<-24, OFFSET_HOURS, 24>::OK;

        Self(
            DateTimeDefaultNow::<Utc>::default()
                .with_timezone(&FixedOffset::east_opt(OFFSET_HOURS * 3600).unwrap()),
        )
    }
}

impl Default for DateTimeDefaultNow<Local, 0> {
    #[cfg(not(test))]
    fn default() -> Self {
        Self(Local::now())
    }

    #[cfg(test)]
    fn default() -> Self {
        Self(
            Local
                .datetime_from_str("2022/10/10 23:40:11.695164300", "%Y/%m/%d %H:%M:%S%.9f")
                .unwrap(),
        )
    }
}

impl Default for DateTimeDefaultNow<Utc, 0> {
    #[cfg(not(test))]
    fn default() -> Self {
        Self(Utc::now())
    }

    #[cfg(test)]
    fn default() -> Self {
        Self(Utc.datetime_from_str(NOW, "%Y/%m/%d %H:%M:%S%.9f").unwrap())
    }
}

impl<Tz, const OFFSET_HOURS: i32> Deref for DateTimeDefaultNow<Tz, OFFSET_HOURS>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    type Target = DateTime<Tz>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Tz, const OFFSET_HOURS: i32> From<DateTime<Tz>> for DateTimeDefaultNow<Tz, OFFSET_HOURS>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    fn from(datetime: DateTime<Tz>) -> Self {
        Self(datetime)
    }
}

impl<const OFFSET_HOURS: i32> std::str::FromStr for DateTimeDefaultNow<FixedOffset, OFFSET_HOURS> {
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> chrono::ParseResult<DateTimeDefaultNow<FixedOffset, OFFSET_HOURS>> {
        DateTime::<FixedOffset>::from_str(s).map(DateTimeDefaultNow::from)
    }
}

impl<const OFFSET_HOURS: i32> std::str::FromStr for DateTimeDefaultNow<Local, OFFSET_HOURS> {
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> chrono::ParseResult<DateTimeDefaultNow<Local, OFFSET_HOURS>> {
        DateTime::<Local>::from_str(s).map(DateTimeDefaultNow::from)
    }
}

impl<const OFFSET_HOURS: i32> std::str::FromStr for DateTimeDefaultNow<Utc, OFFSET_HOURS> {
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> chrono::ParseResult<DateTimeDefaultNow<Utc, OFFSET_HOURS>> {
        DateTime::<Utc>::from_str(s).map(DateTimeDefaultNow::from)
    }
}

impl<Tz, const OFFSET_HOURS: i32> std::cmp::PartialEq<DateTime<Tz>>
    for DateTimeDefaultNow<Tz, OFFSET_HOURS>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    fn eq(&self, other: &DateTime<Tz>) -> bool {
        self.0.eq(other)
    }
}

impl<Tz, const OFFSET_HOURS: i32> std::cmp::PartialEq<DateTimeDefaultNow<Tz, OFFSET_HOURS>>
    for DateTime<Tz>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    fn eq(&self, other: &DateTimeDefaultNow<Tz, OFFSET_HOURS>) -> bool {
        self.eq(&other.0)
    }
}

impl<Tz, const OFFSET_HOURS: i32> std::cmp::PartialOrd<DateTime<Tz>>
    for DateTimeDefaultNow<Tz, OFFSET_HOURS>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    fn partial_cmp(&self, other: &DateTime<Tz>) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl<Tz, const OFFSET_HOURS: i32> std::cmp::PartialOrd<DateTimeDefaultNow<Tz, OFFSET_HOURS>>
    for DateTime<Tz>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    fn partial_cmp(
        &self,
        other: &DateTimeDefaultNow<Tz, OFFSET_HOURS>,
    ) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&other.0)
    }
}

impl<Tz, const OFFSET_HOURS: i32> std::ops::Add<Duration> for DateTimeDefaultNow<Tz, OFFSET_HOURS>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    type Output = DateTimeDefaultNow<Tz, OFFSET_HOURS>;

    #[inline]
    fn add(self, rhs: Duration) -> Self::Output {
        DateTimeDefaultNow(self.0.add(rhs))
    }
}

impl<Tz, const OFFSET_HOURS: i32> std::ops::Add<FixedOffset>
    for DateTimeDefaultNow<Tz, OFFSET_HOURS>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    type Output = DateTimeDefaultNow<Tz, OFFSET_HOURS>;

    #[inline]
    fn add(self, rhs: FixedOffset) -> Self::Output {
        DateTimeDefaultNow(self.0.add(rhs))
    }
}

impl<Tz, const OFFSET_HOURS: i32> std::ops::AddAssign<Duration>
    for DateTimeDefaultNow<Tz, OFFSET_HOURS>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    #[inline]
    fn add_assign(&mut self, rhs: Duration) {
        self.0.add_assign(rhs);
    }
}

impl<Tz, const OFFSET_HOURS: i32> std::ops::Sub<DateTimeDefaultNow<Tz, OFFSET_HOURS>>
    for DateTimeDefaultNow<Tz, OFFSET_HOURS>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    type Output = Duration;

    #[inline]
    fn sub(self, rhs: DateTimeDefaultNow<Tz, OFFSET_HOURS>) -> Duration {
        self.0.sub(rhs.0)
    }
}

impl<Tz, const OFFSET_HOURS: i32> std::ops::Sub<DateTime<Tz>>
    for DateTimeDefaultNow<Tz, OFFSET_HOURS>
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

impl<Tz, const OFFSET_HOURS: i32> std::ops::Sub<Duration> for DateTimeDefaultNow<Tz, OFFSET_HOURS>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    type Output = DateTimeDefaultNow<Tz, OFFSET_HOURS>;

    #[inline]
    fn sub(self, rhs: Duration) -> Self::Output {
        DateTimeDefaultNow(self.0.sub(rhs))
    }
}

impl<Tz, const OFFSET_HOURS: i32> std::ops::Sub<FixedOffset>
    for DateTimeDefaultNow<Tz, OFFSET_HOURS>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    type Output = DateTimeDefaultNow<Tz, OFFSET_HOURS>;

    #[inline]
    fn sub(self, rhs: FixedOffset) -> Self::Output {
        DateTimeDefaultNow(self.0.sub(rhs))
    }
}

impl<Tz, const OFFSET_HOURS: i32> std::ops::SubAssign<Duration>
    for DateTimeDefaultNow<Tz, OFFSET_HOURS>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    #[inline]
    fn sub_assign(&mut self, rhs: Duration) {
        self.0.sub_assign(rhs);
    }
}

impl<Tz, const OFFSET_HOURS: i32> chrono::DurationRound for DateTimeDefaultNow<Tz, OFFSET_HOURS>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: Copy,
{
    type Err = chrono::RoundingError;

    fn duration_round(self, duration: Duration) -> Result<Self, Self::Err> {
        self.0
            .duration_round(duration)
            .map(DateTimeDefaultNow::from)
    }

    fn duration_trunc(self, duration: Duration) -> Result<Self, Self::Err> {
        self.0
            .duration_trunc(duration)
            .map(DateTimeDefaultNow::from)
    }
}

impl<Tz, const OFFSET_HOURS: i32> chrono::Datelike for DateTimeDefaultNow<Tz, OFFSET_HOURS>
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
    fn with_year(&self, year: i32) -> Option<DateTimeDefaultNow<Tz, OFFSET_HOURS>> {
        self.0.with_year(year).map(DateTimeDefaultNow::from)
    }

    #[inline]
    fn with_month(&self, month: u32) -> Option<DateTimeDefaultNow<Tz, OFFSET_HOURS>> {
        self.0.with_month(month).map(DateTimeDefaultNow::from)
    }

    #[inline]
    fn with_month0(&self, month0: u32) -> Option<DateTimeDefaultNow<Tz, OFFSET_HOURS>> {
        self.0.with_month0(month0).map(DateTimeDefaultNow::from)
    }

    #[inline]
    fn with_day(&self, day: u32) -> Option<DateTimeDefaultNow<Tz, OFFSET_HOURS>> {
        self.0.with_day(day).map(DateTimeDefaultNow::from)
    }

    #[inline]
    fn with_day0(&self, day0: u32) -> Option<DateTimeDefaultNow<Tz, OFFSET_HOURS>> {
        self.0.with_day0(day0).map(DateTimeDefaultNow::from)
    }

    #[inline]
    fn with_ordinal(&self, ordinal: u32) -> Option<DateTimeDefaultNow<Tz, OFFSET_HOURS>> {
        self.0.with_ordinal(ordinal).map(DateTimeDefaultNow::from)
    }

    #[inline]
    fn with_ordinal0(&self, ordinal0: u32) -> Option<DateTimeDefaultNow<Tz, OFFSET_HOURS>> {
        self.0.with_ordinal0(ordinal0).map(DateTimeDefaultNow::from)
    }
}

impl<Tz, const OFFSET_HOURS: i32> chrono::Timelike for DateTimeDefaultNow<Tz, OFFSET_HOURS>
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
    fn with_hour(&self, hour: u32) -> Option<DateTimeDefaultNow<Tz, OFFSET_HOURS>> {
        self.0.with_hour(hour).map(DateTimeDefaultNow::from)
    }

    #[inline]
    fn with_minute(&self, min: u32) -> Option<DateTimeDefaultNow<Tz, OFFSET_HOURS>> {
        self.0.with_minute(min).map(DateTimeDefaultNow::from)
    }

    #[inline]
    fn with_second(&self, sec: u32) -> Option<DateTimeDefaultNow<Tz, OFFSET_HOURS>> {
        self.0.with_second(sec).map(DateTimeDefaultNow::from)
    }

    #[inline]
    fn with_nanosecond(&self, nano: u32) -> Option<DateTimeDefaultNow<Tz, OFFSET_HOURS>> {
        self.0.with_nanosecond(nano).map(DateTimeDefaultNow::from)
    }
}

impl<Tz, const OFFSET_HOURS: i32> std::fmt::Display for DateTimeDefaultNow<Tz, OFFSET_HOURS>
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

    use chrono::{DateTime, FixedOffset, Local, TimeZone, Utc};

    use crate::DateTimeDefaultNow;

    #[derive(Default)]
    struct Test {
        updated_at: DateTimeDefaultNow<Utc>,
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
        let datetime = DateTimeDefaultNow::<Utc>::default();

        print_datetime(&datetime)
    }

    #[test]
    fn fixed_offset() {
        let datetime = DateTimeDefaultNow::<FixedOffset, 9>::default();
        println!("{datetime:?}");

        // assert!(false);
    }

    #[test]
    fn use_datetime_method() {
        let datetime = DateTimeDefaultNow::<Utc>::default();

        datetime.date();
    }

    #[test]
    fn convert_from() {
        let now = Local::now();
        let datetime = DateTimeDefaultNow::<Local>::from(now);

        assert_eq!(datetime, now)
    }

    #[test]
    fn convert_into() {
        let now = Local::now();
        let datetime: DateTimeDefaultNow<Local> = now.into();

        assert_eq!(datetime, now)
    }

    #[test]
    fn equation1() {
        let now = Local::now();
        let datetime: DateTimeDefaultNow<Local> = now.into();
        assert!(datetime == now)
    }

    #[test]
    fn equation2() {
        let now = Local::now();
        let datetime: DateTimeDefaultNow<Local> = now.into();
        assert!(now == datetime)
    }

    #[test]
    fn comparing1() {
        assert!(DateTimeDefaultNow::<Local>::default() <= Local::now())
    }

    #[test]
    fn comparing2() {
        assert!(
            Local
                .datetime_from_str("2022/10/10 00:00:00", "%Y/%m/%d %H:%M:%S")
                .unwrap()
                <= DateTimeDefaultNow::<Local>::default()
        )
    }
}
