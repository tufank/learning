use time::Duration;
use time::PrimitiveDateTime as DateTime;
const PERIOD: i64 = 1000000000;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    return start.checked_add(Duration::seconds(PERIOD)).unwrap();
}
