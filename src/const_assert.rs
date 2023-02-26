pub struct AssertOffsetHours<const MIN: i32, const OFFSET_HOURS: i32, const MAX: i32>;

impl<const MIN: i32, const OFFSET_HOURS: i32, const MAX: i32>
    AssertOffsetHours<MIN, OFFSET_HOURS, MAX>
{
    pub const OK: () = assert!(
        MIN < OFFSET_HOURS && OFFSET_HOURS < MAX,
        "FixedOffset::east out of bounds"
    );
}
