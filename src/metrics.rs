use crate::{register_metric, register_metric_vec};
use static_init::{constructor, dynamic};

use prometheus::{Gauge, GaugeVec, IntCounter, IntCounterVec, IntGauge};

register_metric!(
    NODE_BOOT_TIME_SECONDS,
    Gauge,
    node_boot_time_seconds,
    "Node boot time, in unixtime.",
    |state| {
        NODE_BOOT_TIME_SECONDS.set(state.kernel_stats.btime as f64);
    }
);

register_metric!(
    NODE_CONTEXT_SWITCHES_TOTAL,
    IntCounter,
    node_context_switches_total,
    "Total number of context switches.",
    |state| {
        NODE_CONTEXT_SWITCHES_TOTAL
            .inc_by(state.kernel_stats.ctxt - NODE_CONTEXT_SWITCHES_TOTAL.get());
    }
);

register_metric_vec!(
    NODE_CPU_CORE_THROTTLES_TOTAL,
    IntCounterVec,
    node_cpu_core_throttles_total,
    "Number of times this CPU core has been throttled.",
    &["core", "package"],
    |_state| {}
);

register_metric_vec!(
    NODE_CPU_FREQUENCY_MAX_HERZ,
    GaugeVec,
    node_cpu_frequency_max_hertz,
    "Maximum CPU thread frequency in hertz.",
    &["cpu"],
    |_state| {}
);

register_metric!(
    NODE_PROCS_BLOCKED,
    IntGauge,
    node_procs_blocked,
    "Number of processes blocked waiting for I/O to complete.",
    |state| {
        NODE_PROCS_BLOCKED.set(
            state
                .kernel_stats
                .procs_blocked
                .expect("Invalid value for node_procs_blocked") as i64,
        )
    }
);

register_metric!(
    NODE_PROCS_RUNNING,
    IntGauge,
    node_procs_running,
    "Number of processes in runnable state.",
    |state| {
        NODE_PROCS_RUNNING.set(
            state
                .kernel_stats
                .procs_running
                .expect("Invalid value for node_procs_running") as i64,
        );
    }
);
