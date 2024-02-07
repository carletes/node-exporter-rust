use crate::{register_metric, register_metric_vec};
use static_init::{constructor, dynamic};

use prometheus::{GaugeVec, IntCounterVec, IntGauge};

register_metric!(
    NODE_BOOT_TIME_SECONDS,
    IntGauge,
    node_boot_time_seconds,
    "Node boot time, in unixtime.",
    |state| {
        // By casting down from `u64` to `i64` the longest representable boot time goes down to
        // `i64::MAX` / (86400 * 365.4) years, which is a long time.
        NODE_BOOT_TIME_SECONDS.set(i64::try_from(state.boot_time())?);
        Ok(())
    }
);

register_metric!(
    NODE_CONTEXT_SWITCHES_TOTAL,
    IntGauge,
    node_context_switches_total,
    "Total number of context switches.",
    |state| {
        // By casting down from `u64` to `i64` the longest representable number of context switches
        // since boot time goes down to `i64::MAX` / (86400 * 365.4), which I naively believe to be
        // enough.
        NODE_CONTEXT_SWITCHES_TOTAL.set(i64::try_from(state.context_switches())?);
        Ok(())
    }
);

register_metric_vec!(
    NODE_CPU_CORE_THROTTLES_TOTAL,
    IntCounterVec,
    node_cpu_core_throttles_total,
    "Number of times this CPU core has been throttled.",
    &["core", "package"],
    |_state| { Ok(()) }
);

register_metric_vec!(
    NODE_CPU_FREQUENCY_MAX_HERZ,
    GaugeVec,
    node_cpu_frequency_max_hertz,
    "Maximum CPU thread frequency in hertz.",
    &["cpu"],
    |_state| { Ok(()) }
);

register_metric_vec!(
    NODE_CPU_FREQUENCY_MIN_HERZ,
    GaugeVec,
    node_cpu_frequency_min_hertz,
    "Minimum CPU thread frequency in hertz.",
    &["cpu"],
    |_state| { Ok(()) }
);

register_metric!(
    NODE_PROCS_BLOCKED,
    IntGauge,
    node_procs_blocked,
    "Number of processes blocked waiting for I/O to complete.",
    |state| {
        match state.procs_blocked() {
            Some(value) => {
                NODE_PROCS_BLOCKED.set(value.into());
            }
            None => {
                // I _think_ a `None` here means that we do not know how many processes are in this
                // state, so we cannot return a meaningful value for this metric
            }
        }
        Ok(())
    }
);

register_metric!(
    NODE_PROCS_RUNNING,
    IntGauge,
    node_procs_running,
    "Number of processes in runnable state.",
    |state| {
        match state.procs_running() {
            Some(value) => {
                NODE_PROCS_BLOCKED.set(value.into());
            }
            None => {
                // I _think_ a `None` here means that we do not know how many processes are in this
                // state, so we cannot return a meaningful value for this metric
            }
        }
        Ok(())
    }
);
