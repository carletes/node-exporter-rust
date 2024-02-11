use crate::register_metric_vec;

use prometheus::GaugeVec;

register_metric_vec!(
    node_cpu_frequency_max_hertz,
    GaugeVec,
    "Maximum CPU thread frequency in hertz.",
    &["cpu"],
    |_state, _m| { Ok(()) }
);

register_metric_vec!(
    node_cpu_frequency_min_hertz,
    GaugeVec,
    "Minimum CPU thread frequency in hertz.",
    &["cpu"],
    |_state, _m| { Ok(()) }
);
