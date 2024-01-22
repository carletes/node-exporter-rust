use lazy_static::lazy_static;
use prometheus::{register_gauge_vec, register_int_counter_vec, GaugeVec, IntCounterVec, Opts};

use crate::State;

lazy_static! {
    static ref CORE_THROTTLES_TOTAL: IntCounterVec = register_int_counter_vec!(
        Opts::new(
            "node_cpu_core_throttles_total",
            "Number of times this CPU core has been throttled."
        ),
        &["core", "package"]
    )
    .unwrap();
    static ref FREQUENCY_MAX_HERTZ: GaugeVec = register_gauge_vec!(
        Opts::new(
            "node_cpu_frequency_max_hertz",
            "Maximum CPU thread frequency in hertz."
        ),
        &["cpu"]
    )
    .unwrap();
}

pub fn update(_state: &State) {}
