const MAX_PLAUSIBLE_RELEASE_LEAD_NS: i64 = 2_000_000_000;

pub fn frame_is_due(frame_timestamp_ns: i64, now_ns: i64) -> bool {
    let lead = frame_timestamp_ns - now_ns;
    lead <= 0 || lead > MAX_PLAUSIBLE_RELEASE_LEAD_NS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_released_for_future_time_is_held() {
        assert!(!frame_is_due(1_050_000_000, 1_000_000_000));
    }

    #[test]
    fn frame_at_or_past_release_time_is_due() {
        assert!(frame_is_due(1_000_000_000, 1_000_000_000));
        assert!(frame_is_due(900_000_000, 1_000_000_000));
    }

    #[test]
    fn implausible_future_timestamp_is_shown_immediately() {
        assert!(frame_is_due(9_000_000_000_000, 1_000_000_000));
    }
}
