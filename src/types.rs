use serde::{Deserialize, Serialize};

use crate::indicators::supertrend::Trend;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MTFTrend {
    Up,
    Down,
    Neutral,
}

impl MTFTrend {
    pub fn from_trend(trend: Option<Trend>) -> Self {
        match trend {
            Some(Trend::Up) => MTFTrend::Up,
            Some(Trend::Down) => MTFTrend::Down,
            None => MTFTrend::Neutral,
        }
    }

    // pub fn combine(&self, other: MTFTrend) -> MTFTrend {
    //     match (self, other) {
    //         (MTFTrend::Up, MTFTrend::Up) => MTFTrend::Up,
    //         (MTFTrend::Down, MTFTrend::Down) => MTFTrend::Down,
    //         _ => MTFTrend::Neutral,
    //     }
    // }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AllowedTimeframe {
    #[serde(rename = "1m")]
    M1,
    #[serde(rename = "5m")]
    M5,
    #[serde(rename = "15m")]
    M15,
    #[serde(rename = "30m")]
    M30,
    #[serde(rename = "1h")]
    H1,
    #[serde(rename = "4h")]
    H4,
    #[serde(rename = "1d")]
    D1,
}

impl AllowedTimeframe {
    pub fn to_minutes(&self) -> usize {
        match self {
            AllowedTimeframe::M1 => 1,
            AllowedTimeframe::M5 => 5,
            AllowedTimeframe::M15 => 15,
            AllowedTimeframe::M30 => 30,
            AllowedTimeframe::H1 => 60,
            AllowedTimeframe::H4 => 240,
            AllowedTimeframe::D1 => 1440,
        }
    }

    pub fn to_milliseconds(&self) -> usize {
        self.to_minutes() * 60 * 1000
    }

    pub fn floor_timestamp(&self, timestamp: i64) -> i64 {
        let timeframe_ms = self.to_milliseconds() as i64;
        timestamp - (timestamp % timeframe_ms)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_minutes_all_timeframes() {
        assert_eq!(AllowedTimeframe::M1.to_minutes(), 1);
        assert_eq!(AllowedTimeframe::M5.to_minutes(), 5);
        assert_eq!(AllowedTimeframe::M15.to_minutes(), 15);
        assert_eq!(AllowedTimeframe::M30.to_minutes(), 30);
        assert_eq!(AllowedTimeframe::H1.to_minutes(), 60);
        assert_eq!(AllowedTimeframe::H4.to_minutes(), 240);
        assert_eq!(AllowedTimeframe::D1.to_minutes(), 1440);
    }

    #[test]
    fn test_to_milliseconds_all_timeframes() {
        assert_eq!(AllowedTimeframe::M1.to_milliseconds(), 60_000);
        assert_eq!(AllowedTimeframe::M5.to_milliseconds(), 300_000);
        assert_eq!(AllowedTimeframe::M15.to_milliseconds(), 900_000);
        assert_eq!(AllowedTimeframe::M30.to_milliseconds(), 1_800_000);
        assert_eq!(AllowedTimeframe::H1.to_milliseconds(), 3_600_000);
        assert_eq!(AllowedTimeframe::H4.to_milliseconds(), 14_400_000);
        assert_eq!(AllowedTimeframe::D1.to_milliseconds(), 86_400_000);
    }

    #[test]
    fn test_floor_timestamp_m1_exact_boundary() {
        // Timestamp at exact minute boundary
        let timestamp = 60_000; // 1 minute in ms
        assert_eq!(AllowedTimeframe::M1.floor_timestamp(timestamp), 60_000);
    }

    #[test]
    fn test_floor_timestamp_m1_with_offset() {
        // Timestamp with seconds offset
        let timestamp = 90_500; // 1 minute + 30.5 seconds
        assert_eq!(AllowedTimeframe::M1.floor_timestamp(timestamp), 60_000);
    }

    #[test]
    fn test_floor_timestamp_m5_exact_boundary() {
        let timestamp = 300_000; // 5 minutes in ms
        assert_eq!(AllowedTimeframe::M5.floor_timestamp(timestamp), 300_000);
    }

    #[test]
    fn test_floor_timestamp_m5_with_offset() {
        let timestamp = 450_000; // 7.5 minutes
        assert_eq!(AllowedTimeframe::M5.floor_timestamp(timestamp), 300_000);
    }

    #[test]
    fn test_floor_timestamp_h1_exact_boundary() {
        let timestamp = 3_600_000; // 1 hour in ms
        assert_eq!(AllowedTimeframe::H1.floor_timestamp(timestamp), 3_600_000);
    }

    #[test]
    fn test_floor_timestamp_h1_with_offset() {
        let timestamp = 5_400_000; // 1.5 hours
        assert_eq!(AllowedTimeframe::H1.floor_timestamp(timestamp), 3_600_000);
    }

    #[test]
    fn test_floor_timestamp_h4_with_offset() {
        let timestamp = 20_000_000; // ~5.5 hours
        assert_eq!(AllowedTimeframe::H4.floor_timestamp(timestamp), 14_400_000);
    }

    #[test]
    fn test_floor_timestamp_d1_with_offset() {
        let timestamp = 100_000_000; // ~27.7 hours
        assert_eq!(AllowedTimeframe::D1.floor_timestamp(timestamp), 86_400_000);
    }

    #[test]
    fn test_floor_timestamp_zero() {
        // Zero timestamp should return zero for all timeframes
        assert_eq!(AllowedTimeframe::M1.floor_timestamp(0), 0);
        assert_eq!(AllowedTimeframe::M5.floor_timestamp(0), 0);
        assert_eq!(AllowedTimeframe::H1.floor_timestamp(0), 0);
        assert_eq!(AllowedTimeframe::D1.floor_timestamp(0), 0);
    }

    #[test]
    fn test_floor_timestamp_real_world_example() {
        // Real-world timestamp: 2024-01-01 12:34:56.789 UTC
        let timestamp = 1_704_114_896_789;

        // For M5 (5 minutes = 300,000 ms)
        let floored = AllowedTimeframe::M5.floor_timestamp(timestamp);
        assert!(floored <= timestamp);
        assert!(timestamp - floored < 300_000);
        assert_eq!(floored % 300_000, 0);
    }

    #[test]
    fn test_floor_timestamp_consistency() {
        // Floor should be idempotent
        let timestamp = 12_345_678;
        let floored_once = AllowedTimeframe::M15.floor_timestamp(timestamp);
        let floored_twice = AllowedTimeframe::M15.floor_timestamp(floored_once);
        assert_eq!(floored_once, floored_twice);
    }

    #[test]
    fn test_floor_timestamp_all_timeframes() {
        let timestamp = 50_000_000; // ~13.9 hours

        // Verify each timeframe floors correctly
        assert_eq!(AllowedTimeframe::M1.floor_timestamp(timestamp) % 60_000, 0);
        assert_eq!(AllowedTimeframe::M5.floor_timestamp(timestamp) % 300_000, 0);
        assert_eq!(
            AllowedTimeframe::M15.floor_timestamp(timestamp) % 900_000,
            0
        );
        assert_eq!(
            AllowedTimeframe::M30.floor_timestamp(timestamp) % 1_800_000,
            0
        );
        assert_eq!(
            AllowedTimeframe::H1.floor_timestamp(timestamp) % 3_600_000,
            0
        );
        assert_eq!(
            AllowedTimeframe::H4.floor_timestamp(timestamp) % 14_400_000,
            0
        );
        assert_eq!(
            AllowedTimeframe::D1.floor_timestamp(timestamp) % 86_400_000,
            0
        );
    }
}
