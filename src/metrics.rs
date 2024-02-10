use crate::{register_metric, register_metric_vec};

use anyhow::anyhow;
use prometheus::{GaugeVec, IntCounter, IntCounterVec, IntGauge};

register_metric!(
    node_boot_time_seconds,
    IntGauge,
    "Node boot time, in unixtime.",
    |state, m| {
        let g: IntGauge = m.try_into()?;
        // By casting down from `u64` to `i64` the longest representable boot time goes down to
        // `i64::MAX` / (86400 * 365.4) years, which is a long time.
        g.set(i64::try_from(state.boot_time())?);
        Ok(())
    }
);

#[cfg(test)]
mod tests_boot_time {
    use crate::tests::{dump_with_state, MockSystemState};

    #[test]
    fn valid() -> crate::Result<()> {
        let mut state = MockSystemState::default();
        state.boot_time = 42;
        let (m, _) = dump_with_state(&state)?;
        assert!(m.contains("node_boot_time_seconds 42"), "Metrics: <{}>", m);
        Ok(())
    }

    #[test]
    fn too_large() -> crate::Result<()> {
        let mut state = MockSystemState::default();
        state.boot_time = (i64::MAX as u64) + 1;
        let (m, err) = dump_with_state(&state)?;
        assert!(m.contains("node_boot_time_seconds "), "Metrics: <{}>", m);
        assert!(
            err.contains(
                &"node_boot_time_seconds: out of range integral type conversion attempted"
                    .to_string()
            ),
            "Errors: {:?}",
            err
        );
        Ok(())
    }
}

register_metric!(
    node_context_switches_total,
    IntCounter,
    "Total number of context switches.",
    |state, m| {
        let c: IntCounter = m.try_into()?;
        let cur = c.get();
        let new = state.context_switches();
        if new < cur {
            Err(anyhow!(
                "Number of context switches {} lesser than current metric value {}",
                new,
                cur
            ))
        } else {
            c.reset();
            c.inc_by(new);
            Ok(())
        }
    }
);

#[cfg(test)]
mod tests_context_switches {
    use crate::tests::{dump_with_state, MockSystemState};

    #[test]
    fn valid() -> crate::Result<()> {
        let mut state = MockSystemState::default();
        state.context_switches = 42;
        let (m, _) = dump_with_state(&state)?;
        assert!(
            m.contains("node_context_switches_total 42"),
            "Metrics: <{}>",
            m
        );
        Ok(())
    }

    #[test]
    fn unexpected_downcount() -> crate::Result<()> {
        let mut state = MockSystemState::default();
        state.context_switches = 42;
        let (m, _) = dump_with_state(&state)?;
        assert!(
            m.contains("node_context_switches_total 42"),
            "Metrics: <{}>",
            m
        );

        state.context_switches = 41;
        let (m, err) = dump_with_state(&state)?;
        assert!(
            m.contains("node_context_switches_total 42"),
            "Metrics: <{}>",
            m
        );
        assert!(
            err.contains(
                &"node_context_switches_total: Number of context switches 41 lesser than current metric value 42"
                    .to_string()
            ),
            "Errors: {:?}",
            err
        );
        Ok(())
    }
}

register_metric_vec!(
    node_cpu_core_throttles_total,
    IntCounterVec,
    "Number of times this CPU core has been throttled.",
    &["core", "package"],
    |_state, _m| { Ok(()) }
);

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

register_metric!(
    node_procs_blocked,
    IntGauge,
    "Number of processes blocked waiting for I/O to complete.",
    |state, m| {
        let g: IntGauge = m.try_into()?;
        match state.procs_blocked() {
            Some(value) => {
                g.set(value.into());
            }
            None => {
                // I _think_ a `None` here means that we do not know how many processes are in this
                // state, so we cannot return a meaningful value for this metric
            }
        }
        Ok(())
    }
);

#[cfg(test)]
mod tests_procs_blocked {
    use crate::tests::{dump_with_state, MockSystemState};

    #[test]
    fn unavailable() -> crate::Result<()> {
        let mut state = MockSystemState::default();
        state.procs_blocked = None;
        let (m, err) = dump_with_state(&state)?;
        assert!(m.contains("node_procs_blocked 0"), "Metrics: {}", m);
        assert!(
            !err.contains(&"node_procs_blocked:".to_string()),
            "Errors: {:?}",
            err
        );
        Ok(())
    }
}

register_metric!(
    node_procs_running,
    IntGauge,
    "Number of processes in runnable state.",
    |state, m| {
        let g: IntGauge = m.try_into()?;
        match state.procs_running() {
            Some(value) => {
                g.set(value.into());
            }
            None => {
                // I _think_ a `None` here means that we do not know how many processes are in this
                // state, so we cannot return a meaningful value for this metric
            }
        }
        Ok(())
    }
);

#[cfg(test)]
mod tests_procs_running {
    use crate::tests::{dump_with_state, MockSystemState};

    #[test]
    fn unavailable() -> crate::Result<()> {
        let mut state = MockSystemState::default();
        state.procs_running = None;
        let (m, err) = dump_with_state(&state)?;
        assert!(m.contains("node_procs_running 0"), "Metrics: {}", m);
        assert!(
            !err.contains(&"node_procs_running:".to_string()),
            "Errors: {:?}",
            err
        );
        Ok(())
    }
}
