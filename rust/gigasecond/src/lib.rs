use time::Duration;

use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let giga_second = Duration::new(1_000_000_000, 0);
    start.checked_add(giga_second).unwrap()
}
