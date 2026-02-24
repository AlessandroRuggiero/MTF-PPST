mod indicators;
mod outcomes_matrix;
mod ppst;
mod types;

use exchange_outpost_abi::FunctionArgs;
use extism_pdk::{FnResult, Json, ToBytes, encoding, plugin_fn};
use outcomes_matrix::OutcomesMatrix;
use serde::{Deserialize, Serialize};
use types::AllowedTimeframe;

#[derive(Deserialize, ToBytes)]
#[encoding(Json)]
pub struct PPSTConfig {
    pivot_point_period: usize,
    atr_factor: f64,
    atr_period: usize,
    timeframe: AllowedTimeframe,
    candles_key: String,
}

#[derive(Serialize, ToBytes)]
#[encoding(Json)]
pub struct Output {
    outcomes_matrix: OutcomesMatrix,
}

#[plugin_fn]
pub fn run(call_args: FunctionArgs) -> FnResult<Output> {
    let ppst_configs: Vec<PPSTConfig> = call_args.get_call_argument("ppstConfigs")?;
    let ppsts: Vec<ppst::PPST> = ppst_configs
        .into_iter()
        .map(|config| {
            let candles = call_args
                .get_candles(&config.candles_key)
                .expect("Failed to get candles"); // TODO handle this error properly
            let mut ppst_iteration = ppst::PPST::new(
                config.pivot_point_period,
                config.atr_factor,
                config.atr_period,
                candles.len(),
                config.timeframe,
            );
            ppst_iteration.calculate(candles);
            ppst_iteration
        })
        .collect();
    let mut outcomes_matrix = OutcomesMatrix::new(ppsts.len());
    outcomes_matrix.apply_strategies(ppsts);
    Ok(Output { outcomes_matrix })
}
