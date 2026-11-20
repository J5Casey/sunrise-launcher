use super::*;

/// A release's day as the mockup writes it ("Nov 1"), with its time in UTC when known.
pub(super) fn published(date: &str, time: &str) -> String {
    let day = sunrise_launcher::nightly::day(date);
    if time.is_empty() {
        day
    } else {
        trf("{day}, {time} UTC", &[("day", &day), ("time", time)])
    }
}
