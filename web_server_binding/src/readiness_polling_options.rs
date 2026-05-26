use std::time::Duration;

#[derive(Clone)]
pub struct ReadinessPollingOptions {
    pub poll_interval: Duration,
    pub readiness_timeout: Duration,
}

impl Default for ReadinessPollingOptions {
    fn default() -> Self {
        Self {
            poll_interval: Duration::from_millis(1),
            readiness_timeout: Duration::from_secs(3),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::ReadinessPollingOptions;

    #[test]
    fn default_uses_one_millisecond_poll_and_three_second_timeout() {
        let ReadinessPollingOptions {
            poll_interval,
            readiness_timeout,
        } = ReadinessPollingOptions::default();

        assert_eq!(poll_interval, Duration::from_millis(1));
        assert_eq!(readiness_timeout, Duration::from_secs(3));
    }

    #[test]
    fn can_be_cloned() {
        let original = ReadinessPollingOptions {
            poll_interval: Duration::from_millis(7),
            readiness_timeout: Duration::from_millis(11),
        };
        let cloned = original.clone();

        assert_eq!(original.poll_interval, cloned.poll_interval);
        assert_eq!(original.readiness_timeout, cloned.readiness_timeout);
    }
}
