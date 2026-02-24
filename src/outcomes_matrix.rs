use std::collections::BTreeMap;

use crate::{ppst, types::MTFTrend};
use ndarray::Array2;
pub struct OutcomesMatrix {
    pub stategies: Vec<ppst::PPST>,
    pub outcomes: BTreeMap<i64, Vec<MTFTrend>>,
}

impl OutcomesMatrix {
    pub fn new(n: usize) -> Self {
        let outcomes = Array2::from_elem((n, master_candles_open_time.len()), MTFTrend::Neutral);
        OutcomesMatrix {
            stategies: Vec::with_capacity(n),
            outcomes: BTreeMap::new(),
        }
    }

    pub fn apply_strategies(&mut self, mut strategies: Vec<ppst::PPST>) {
        strategies.sort_by_key(|s| s.timeframe); // Ensure strategies are ordered by timeframe
        for (n, strategy) in strategies.into_iter().enumerate() {
            if n == 0 {
                // Initialize the b tree with the timestamps from the first strategy
                for (idx, timestamp) in strategy.candles_trend.keys().enumerate() {
                    let mut trends = vec![MTFTrend::Neutral; strategies.len()];
                    trends[n] =
                        MTFTrend::from_trend(strategy.candles_trend.get(timestamp).cloned());
                    self.outcomes.insert(*timestamp, trends);
                }
            } else {
                // For subsequent strategies, update the existing entries in the b tree
            }
        }
    }
}
