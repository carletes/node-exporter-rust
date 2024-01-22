use lazy_static::lazy_static;
use prometheus::{register_gauge, register_int_counter, Gauge, IntCounter};

use crate::State;

mod cpu;
mod disk;

lazy_static! {
    static ref BOOT_TIME_SECONDS: Gauge =
        register_gauge!("node_boot_time_seconds", "Node boot time, in unixtime.").unwrap();
    static ref CONTEXT_SWITCHES_TOTAL: IntCounter = register_int_counter!(
        "node_context_switches_total",
        "Total number of context switches."
    )
    .unwrap();
    static ref PROCS_BLOCKED: Gauge = register_gauge!(
        "node_procs_blocked",
        "Number of processes blocked waiting for I/O to complete."
    )
    .unwrap();
    static ref PROCS_RUNNING: Gauge = register_gauge!(
        "node_procs_running",
        "Number of processes in runnable state."
    )
    .unwrap();
}

pub fn update(state: &State) {
    cpu::update(state);
    disk::update(state);

    BOOT_TIME_SECONDS.set(state.kernel_stats.btime as f64);

    CONTEXT_SWITCHES_TOTAL.inc_by(state.kernel_stats.ctxt - CONTEXT_SWITCHES_TOTAL.get());

    PROCS_BLOCKED.set(state.kernel_stats.procs_blocked.unwrap() as f64);

    PROCS_RUNNING.set(state.kernel_stats.procs_running.unwrap() as f64);
}
