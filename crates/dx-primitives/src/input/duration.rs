use std::time::Duration;

use dioxus::prelude::*;

#[component]
pub fn DurationInput(value: Duration, #[props(default)] onchange: Callback<Duration>) -> Element {
    let mut parts = DurationParts::from(value);

    let onchange_hrs = move |e: Event<FormData>| {
        if let Ok(hrs) = e.value().parse() {
            // let mut parts = parts.write();
            parts.hours = hrs;
            onchange.call(Duration::from(parts));
        }
    };

    let onchange_mins = move |e: Event<FormData>| {
        if let Ok(mins) = e.value().parse() {
            // let mut parts = parts.write();
            parts.minutes = mins;
            onchange.call(Duration::from(parts));
        }
    };

    let onchange_secs = move |e: Event<FormData>| {
        if let Ok(secs) = e.value().parse() {
            // let mut parts = parts.write();
            parts.seconds = secs;
            onchange.call(Duration::from(parts));
        }
    };

    let onchange_millis = move |e: Event<FormData>| {
        if let Ok(millis) = e.value().parse() {
            // let mut parts = parts.write();
            parts.millis = millis;
            onchange.call(Duration::from(parts));
        }
    };

    let hrs = parts.hours;
    let mins = parts.minutes;
    let secs = parts.seconds;
    let millis = parts.millis;

    info!("time parts: {:?}", parts);

    rsx! {
        div { class: "flex flex-row w-full gap-2",
            div { class: "flex flex-col grow-0",
                label { class: "", "H" }
                input { class: "" , type: "number", value: hrs, min: 0, max: 24, onchange: onchange_hrs }
            }
            div { class: "flex flex-col grow-0",
                label { class: "", "M" }
                input { class: "", type: "number", value: mins, min: 0, max: 60, onchange: onchange_mins}
            }
            div { class: "flex flex-col grow-0",
                label { class: "", "S" }
                input { class: "", type: "number", value: secs, min: 0, max: 60, onchange: onchange_secs }
            }
            div { class: "flex flex-col grow-0",
                label { class: "", "ms" }
                input { class: "", type: "number", value: millis, min: 0, max: 1000, onchange: onchange_millis }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DurationParts {
    days: u64,
    hours: u64,
    minutes: u64,
    seconds: u64,
    millis: u64,
}

impl From<Duration> for DurationParts {
    fn from(value: Duration) -> Self {
        let total_millis = value.as_millis() as u64;
        let days = total_millis / (24 * 60 * 60 * 1000);
        let hours = (total_millis % (24 * 60 * 60 * 1000)) / (60 * 60 * 1000);
        let minutes = (total_millis % (60 * 60 * 1000)) / (60 * 1000);
        let seconds = (total_millis % (60 * 1000)) / 1000;
        let millis = total_millis % 1000;

        DurationParts {
            days,
            hours,
            minutes,
            seconds,
            millis,
        }
    }
}

impl From<DurationParts> for Duration {
    fn from(value: DurationParts) -> Self {
        let total_millis = value.days * 24 * 60 * 60 * 1000
            + value.hours * 60 * 60 * 1000
            + value.minutes * 60 * 1000
            + value.seconds * 1000
            + value.millis;

        Duration::from_millis(total_millis as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn duration_parts() {
        // Test conversion from Duration to DurationParts and back
        let original_duration = Duration::from_millis(93784567); // 1 day, 2 hours, 3 minutes, 4 seconds, 567 milliseconds

        let parts: DurationParts = original_duration.into();
        assert_eq!(parts.days, 1);
        assert_eq!(parts.hours, 2);
        assert_eq!(parts.minutes, 3);
        assert_eq!(parts.seconds, 4);
        assert_eq!(parts.millis, 567);

        let converted_back: Duration = parts.into();
        assert_eq!(converted_back, original_duration);

        // Test edge case: zero duration
        let zero_duration = Duration::from_millis(0);
        let zero_parts: DurationParts = zero_duration.into();
        assert_eq!(zero_parts.days, 0);
        assert_eq!(zero_parts.hours, 0);
        assert_eq!(zero_parts.minutes, 0);
        assert_eq!(zero_parts.seconds, 0);
        assert_eq!(zero_parts.millis, 0);

        let zero_back: Duration = zero_parts.into();
        assert_eq!(zero_back, zero_duration);
    }
}
