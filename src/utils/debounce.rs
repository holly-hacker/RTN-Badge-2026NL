use embassy_time::{Duration, Instant};

pub struct Debounce<T> {
    /// The last "valid" or "stable" value that was reported
    stable: T,
    /// The last bounce (ie. change in measurement) that was reported from the update function
    last_bounce: (T, Instant),
}

impl<T> Debounce<T>
where
    T: PartialEq + Copy,
{
    // could be configurable
    const DEBOUNCE: Duration = Duration::from_millis(10);

    pub fn new(initial: T) -> Self {
        Self {
            stable: initial,
            last_bounce: (initial, Instant::MIN),
        }
    }

    pub fn tick(&mut self, measurement: T) -> DebounceResult<T> {
        let now = Instant::now();

        // Did the measurement change from last tick? (ie. is there a bounce?)
        if self.last_bounce.0 != measurement {
            // New bounce, store it and report old stable value. This resets the bounce timer so we
            // can't logically report a changed value.
            self.last_bounce = (measurement, now);

            return DebounceResult::Unchanged(self.stable);
        }

        // The measurement did not change this tick.
        // Check if the current value is the same as the stable value.
        if self.last_bounce.0 == self.stable {
            // Current value is the same as the stable value, meaning there's nothing to do. We
            // can't debounce into the same value we had before.
            return DebounceResult::Unchanged(self.stable);
        }

        // The current measurement differs from the last stable measurement. See if enough time has
        // passed to report this as a change.
        let time_since_last_bounce = now - self.last_bounce.1;
        if time_since_last_bounce < Self::DEBOUNCE {
            // Not enough time has passed, report no change.
            return DebounceResult::Unchanged(self.stable);
        }

        // Enough time has passed since the last bounce, report the changed value
        let old_stable = core::mem::replace(&mut self.stable, measurement);
        DebounceResult::Changed(old_stable, self.stable)
    }
}

pub enum DebounceResult<T> {
    Unchanged(T),
    Changed(T, T),
}

impl<T> DebounceResult<T> {
    pub fn current(&self) -> &T {
        match self {
            DebounceResult::Unchanged(v) => v,
            DebounceResult::Changed(_, v) => v,
        }
    }
}
