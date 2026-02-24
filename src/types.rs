use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MTFTrend {
    Up,
    Down,
    Neutral,
}

impl MTFTrend {
    pub fn from_trend(trend: Optional<Trend>) -> Self {
        match trend {
            Some(Trend::Up) => MTFTrend::Up,
            Some(Trend::Down) => MTFTrend::Down,
            None => MTFTrend::Neutral,
        }
    }
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
