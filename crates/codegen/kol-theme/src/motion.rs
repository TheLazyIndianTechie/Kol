use std::time::Duration;

/// Prompt cursor blink. Prototype: `@keyframes kolblink`, `1s step-end infinite`.
pub const BLINK: Duration = Duration::from_millis(1000);

/// Rail scan sweep, top to bottom. Prototype: `@keyframes kolsweep`, `7s linear infinite`.
pub const SWEEP: Duration = Duration::from_millis(7000);

/// Idle `STANDING BY` pulse. Prototype: `kolpulse 3.4s ease-in-out infinite`.
pub const PULSE_IDLE: Duration = Duration::from_millis(3400);

/// Recording indicator pulse. Prototype: `kolpulse 1.4s ease-in-out infinite`.
pub const PULSE_REC: Duration = Duration::from_millis(1400);

/// Selection slide. Prototype: `transition .18s cubic-bezier(.2,.8,.2,1)`.
pub const SLIDE: Duration = Duration::from_millis(180);

/// One named timing token, with the design rule it came from.
#[derive(Debug, Clone, Copy)]
pub struct Timing {
    pub name: &'static str,
    pub duration: Duration,
    /// The CSS rule in the prototype this was transcribed from.
    pub source: &'static str,
    /// What it animates.
    pub note: &'static str,
}

/// Every timing token, in card order.
pub const TIMINGS: &[Timing] = &[
    Timing {
        name: "BLINK",
        duration: BLINK,
        source: "kolblink 1s step-end infinite",
        note: "Prompt cursor",
    },
    Timing {
        name: "SWEEP",
        duration: SWEEP,
        source: "kolsweep 7s linear infinite",
        note: "Rail scan line",
    },
    Timing {
        name: "PULSE_IDLE",
        duration: PULSE_IDLE,
        source: "kolpulse 3.4s ease-in-out infinite",
        note: "STANDING BY breathing",
    },
    Timing {
        name: "PULSE_REC",
        duration: PULSE_REC,
        source: "kolpulse 1.4s ease-in-out infinite",
        note: "Recording indicator",
    },
    Timing {
        name: "SLIDE",
        duration: SLIDE,
        source: "transition .18s cubic-bezier(.2,.8,.2,1)",
        note: "Selection movement",
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recording_pulse_is_faster_than_idle() {
        assert!(PULSE_REC < PULSE_IDLE, "urgency reads as tempo");
    }

    #[test]
    fn every_timing_records_its_design_source() {
        for t in TIMINGS {
            assert!(!t.source.is_empty(), "{} has no design source", t.name);
        }
    }

    #[test]
    fn timings_list_is_complete() {
        assert_eq!(TIMINGS.len(), 5);
    }
}
