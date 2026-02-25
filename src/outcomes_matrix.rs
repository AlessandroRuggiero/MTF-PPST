use extism_pdk::info;
use serde::Serialize;

use crate::types::AllowedTimeframe;
use crate::{ppst, types::MTFTrend};
use std::collections::BTreeMap;

#[derive(Serialize)]
pub struct OutcomesMatrix {
    pub stategies: Vec<ppst::PPST>,
    pub outcomes: BTreeMap<i64, Vec<MTFTrend>>,
    pub base_timeframe: Option<AllowedTimeframe>,
}

impl OutcomesMatrix {
    pub fn new(n: usize) -> Self {
        OutcomesMatrix {
            stategies: Vec::with_capacity(n),
            outcomes: BTreeMap::new(),
            base_timeframe: None,
        }
    }

    pub fn apply_strategies(&mut self, mut strategies: Vec<ppst::PPST>) {
        let num_strategies = strategies.len();
        strategies.sort_by_key(|s| s.timeframe); // Ensure strategies are ordered by timeframe
        for (n, strategy) in strategies.into_iter().enumerate() {
            if n == 0 {
                self.base_timeframe = Some(strategy.timeframe);
                // Initialize the b tree with the timestamps from the first strategy
                for (_, timestamp) in strategy.candles_trend.keys().enumerate() {
                    let mut trends = vec![MTFTrend::Neutral; num_strategies];
                    trends[n] =
                        MTFTrend::from_trend(strategy.candles_trend.get(timestamp).cloned());
                    self.outcomes.insert(*timestamp, trends);
                }
            } else {
                // For subsequent strategies, update the existing entries in the b tree
                for (timestamp, trends) in self.outcomes.iter_mut() {
                    if strategy.timeframe.to_minutes()
                        > self
                            .base_timeframe
                            .expect("Base timeframe not set")
                            .to_minutes()
                    {
                        // it means this is a higher timeframe strategy, so we need to find the corresponding trend for the current timestamp
                        let htf_candle_start = strategy.timeframe.floor_timestamp(*timestamp);
                        let previous_htf_candle_start =
                            htf_candle_start - strategy.timeframe.to_milliseconds() as i64;
                        let htf_trend = strategy.candles_trend.get(&previous_htf_candle_start);
                        if let Some(trend) = htf_trend {
                            trends[n] = MTFTrend::from_trend(Some(*trend));
                        } else {
                            // This means we would have to look for previous candles than the start of the data
                            info!(
                                "No trend found for timestamp {} in strategy with timeframe {:?}. This may be due to missing data for higher timeframes. (Its expected)",
                                timestamp, strategy.timeframe
                            );
                            trends[n] = MTFTrend::Neutral;
                        }
                    }
                }
            }
            self.stategies.push(strategy);
        }
    }
}
