use std::collections::HashMap;

use crate::register_metric_vec;

use prometheus::{CounterVec, IntCounterVec};

register_metric_vec!(
    node_cpu_core_throttles_total,
    IntCounterVec,
    "Number of times this CPU core has been throttled.",
    &["core", "package"],
    |state, m| {
        let c: IntCounterVec = m.try_into()?;
        let seen: HashMap<(String, String), u64> = HashMap::new();
        for cpu in &state.cpu_stats().threads {
            let k = (cpu.core_id.to_string(), cpu.physical_package_id.to_string());
            if !seen.contains_key(&k) {
                let (core, package) = k;
                let v = c.get_metric_with_label_values(&[&core, &package])?;
                v.reset();
                v.inc_by(cpu.num_core_throttles);
            }
        }

        Ok(())
    }
);

register_metric_vec!(
    node_cpu_package_throttles_total,
    IntCounterVec,
    "Number of times this CPU package has been throttled.",
    &["package"],
    |state, m| {
        let c: IntCounterVec = m.try_into()?;
        let seen: HashMap<String, u64> = HashMap::new();
        for cpu in &state.cpu_stats().threads {
            let package = cpu.physical_package_id.to_string();
            if !seen.contains_key(&package) {
                let v = c.get_metric_with_label_values(&[&package])?;
                v.reset();
                v.inc_by(cpu.num_package_throttles);
            }
        }

        Ok(())
    }
);

#[cfg(test)]
mod tests_throttles {
    use crate::sysfs::CpuStats;
    use crate::tests::{dump_with_state, MockSystemState};
    use crate::Result;

    #[test]
    fn valid() -> Result<()> {
        let mut state = MockSystemState::default();
        state.cpu_stats = CpuStats::current()?;
        let (m, _) = dump_with_state(&state)?;
        assert!(
            m.contains(r#"node_cpu_core_throttles_total{core="0",package="0"}"#),
            "Metrics: <{}>",
            m
        );
        assert!(
            m.contains(r#"node_cpu_package_throttles_total{package="0"}"#),
            "Metrics: <{}>",
            m
        );
        Ok(())
    }
}

register_metric_vec!(
    node_cpu_seconds_total,
    CounterVec,
    "Seconds the CPUs spent in each mode.",
    &["cpu", "mode"],
    |state, m| {
        let c: CounterVec = m.try_into()?;
        for (i, cpu_time) in state.cpu_time().iter().enumerate() {
            let v = c.get_metric_with_label_values(&[&i.to_string(), "idle"])?;
            v.reset();
            v.inc_by((cpu_time.idle_ms() as f64) / 1000.0);

            if let Some(ms) = cpu_time.iowait_ms() {
                let v = c.get_metric_with_label_values(&[&i.to_string(), "iowait"])?;
                v.reset();
                v.inc_by((ms as f64) / 1000.0);
            }

            if let Some(ms) = cpu_time.irq_ms() {
                let v = c.get_metric_with_label_values(&[&i.to_string(), "irq"])?;
                v.reset();
                v.inc_by((ms as f64) / 1000.0);
            }

            let v = c.get_metric_with_label_values(&[&i.to_string(), "nice"])?;
            v.reset();
            v.inc_by((cpu_time.nice_ms() as f64) / 1000.0);

            if let Some(ms) = cpu_time.softirq_ms() {
                let v = c.get_metric_with_label_values(&[&i.to_string(), "softirq"])?;
                v.reset();
                v.inc_by((ms as f64) / 1000.0);
            }

            if let Some(ms) = cpu_time.steal_ms() {
                let v = c.get_metric_with_label_values(&[&i.to_string(), "steal"])?;
                v.reset();
                v.inc_by((ms as f64) / 1000.0);
            }

            let v = c.get_metric_with_label_values(&[&i.to_string(), "system"])?;
            v.reset();
            v.inc_by((cpu_time.system_ms() as f64) / 1000.0);

            let v = c.get_metric_with_label_values(&[&i.to_string(), "user"])?;
            v.reset();
            v.inc_by((cpu_time.user_ms() as f64) / 1000.0);
        }
        Ok(())
    }
);

#[cfg(test)]
mod tests_cpu_seconds_total {
    use crate::tests::{dump_with_state, MockSystemState};
    use crate::Result;
    use procfs::{CurrentSI, KernelStats};

    #[test]
    fn valid() -> Result<()> {
        let mut state = MockSystemState::default();
        state.cpu_time = KernelStats::current()?.cpu_time;
        let (m, _) = dump_with_state(&state)?;
        assert!(
            m.contains(r#"node_cpu_seconds_total{cpu="0",mode="idle"}"#),
            "Metrics: <{}>",
            m
        );
        assert!(
            m.contains(r#"node_cpu_seconds_total{cpu="0",mode="iowait"}"#),
            "Metrics: <{}>",
            m
        );
        assert!(
            m.contains(r#"node_cpu_seconds_total{cpu="0",mode="irq"}"#),
            "Metrics: <{}>",
            m
        );
        assert!(
            m.contains(r#"node_cpu_seconds_total{cpu="0",mode="nice"}"#),
            "Metrics: <{}>",
            m
        );
        assert!(
            m.contains(r#"node_cpu_seconds_total{cpu="0",mode="softirq"}"#),
            "Metrics: <{}>",
            m
        );
        assert!(
            m.contains(r#"node_cpu_seconds_total{cpu="0",mode="steal"}"#),
            "Metrics: <{}>",
            m
        );
        assert!(
            m.contains(r#"node_cpu_seconds_total{cpu="0",mode="system"}"#),
            "Metrics: <{}>",
            m
        );
        assert!(
            m.contains(r#"node_cpu_seconds_total{cpu="0",mode="user"}"#),
            "Metrics: <{}>",
            m
        );
        Ok(())
    }
}

register_metric_vec!(
    node_cpu_guest_seconds_total,
    CounterVec,
    "Seconds the CPUs spent in guests (VMs) for each mode.",
    &["cpu", "mode"],
    |state, m| {
        let c: CounterVec = m.try_into()?;
        for (i, cpu_time) in state.cpu_time().iter().enumerate() {
            if let Some(ms) = cpu_time.guest_ms() {
                let v = c.get_metric_with_label_values(&[&i.to_string(), "user"])?;
                v.reset();
                v.inc_by((ms as f64) / 1000.0);
            }

            if let Some(ms) = cpu_time.guest_nice_ms() {
                let v = c.get_metric_with_label_values(&[&i.to_string(), "nice"])?;
                v.reset();
                v.inc_by((ms as f64) / 1000.0);
            }
        }
        Ok(())
    }
);

#[cfg(test)]
mod tests_cpu_guest_seconds_total {
    use crate::tests::{dump_with_state, MockSystemState};
    use crate::Result;
    use procfs::{CurrentSI, KernelStats};

    #[test]
    fn valid() -> Result<()> {
        let mut state = MockSystemState::default();
        state.cpu_time = KernelStats::current()?.cpu_time;
        let (m, _) = dump_with_state(&state)?;
        assert!(
            m.contains(r#"node_cpu_guest_seconds_total{cpu="0",mode="user"}"#),
            "Metrics: <{}>",
            m
        );
        assert!(
            m.contains(r#"node_cpu_guest_seconds_total{cpu="0",mode="nice"}"#),
            "Metrics: <{}>",
            m
        );
        Ok(())
    }
}
