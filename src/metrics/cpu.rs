use crate::register_metric_vec;

use prometheus::IntCounterVec;

register_metric_vec!(
    node_cpu_core_throttles_total,
    IntCounterVec,
    "Number of times this CPU core has been throttled.",
    &["core", "package"],
    |_state, _m| { Ok(()) }
);
