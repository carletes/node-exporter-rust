use crate::register_metric_vec;

use prometheus::IntGaugeVec;

macro_rules! register_cpufreq_metric {
    ($ctor: ident, $help: literal, $state_field: ident, $test_mod: ident) => {
        register_metric_vec!($ctor, IntGaugeVec, $help, &["cpu"], |state, m| {
            let g: IntGaugeVec = m.try_into()?;
            for (i, stats) in state.cpu_stats().threads.iter().enumerate() {
                let v = g.get_metric_with_label_values(&[&i.to_string()])?;
                v.set(i64::try_from(stats.$state_field)?);
            }
            Ok(())
        });

        #[cfg(test)]
        mod $test_mod {
            use crate::sysfs::CpuStat;
            use crate::tests::{dump_with_state, MockSystemState};
            use crate::Result;
            use std::default::Default;

            #[test]
            fn valid() -> Result<()> {
                let mut state = MockSystemState::default();
                state.cpu_stats.threads.push(CpuStat {
                    $state_field: 42,
                    ..Default::default()
                });
                state.cpu_stats.threads.push(CpuStat {
                    $state_field: 43,
                    ..Default::default()
                });
                let (m, _) = dump_with_state(&state)?;
                assert!(
                    m.contains(concat!(stringify!($ctor), r#"{cpu="0"}"#, " 42")),
                    "Metrics: <{}>",
                    m
                );
                assert!(
                    m.contains(concat!(stringify!($ctor), r#"{cpu="1"}"#, " 43")),
                    "Metrics: <{}>",
                    m
                );
                Ok(())
            }

            #[test]
            fn too_large() -> Result<()> {
                let mut state = MockSystemState::default();
                state.cpu_stats.threads.push(CpuStat {
                    $state_field: (i64::MAX as u64) + 1,
                    ..Default::default()
                });
                let (m, err) = dump_with_state(&state)?;
                assert!(
                    m.contains(concat!(stringify!($ctor), " ")),
                    "Metrics: <{}>",
                    m
                );
                assert!(
                    err.contains(
                        &format!(
                            "{}: out of range integral type conversion attempted",
                            stringify!($ctor)
                        )
                        .to_string()
                    ),
                    "Errors: {:?}",
                    err
                );
                Ok(())
            }
        }
    };
}

register_cpufreq_metric!(
    node_cpu_frequency_min_hertz,
    "Minimum CPU thread frequency in hertz.",
    min_freq,
    tests_min_freq
);

register_cpufreq_metric!(
    node_cpu_scaling_frequency_hertz,
    "Current scaled CPU thread frequency in hertz.",
    scaling_cur_freq,
    test_scaling_cur_freq
);

register_cpufreq_metric!(
    node_cpu_scaling_frequency_max_hertz,
    "Maximum scaled CPU thread frequency in hertz.",
    scaling_max_freq,
    tests_scaling_max_freq
);

register_cpufreq_metric!(
    node_cpu_scaling_frequency_min_hertz,
    "Minimum scaled CPU thread frequency in hertz.",
    scaling_min_freq,
    tests_scaling_min_freq
);

register_metric_vec!(
    node_cpu_scaling_governor,
    IntGaugeVec,
    "Current enabled CPU frequency governor.",
    &["cpu", "governor"],
    |state, m| {
        let g: IntGaugeVec = m.try_into()?;
        for (i, stats) in state.cpu_stats().threads.iter().enumerate() {
            for gov in stats.scaling_available_governors.iter() {
                let v = g.get_metric_with_label_values(&[&i.to_string(), gov])?;
                if gov == &stats.scaling_governor {
                    v.set(1);
                } else {
                    v.set(0);
                }
            }
        }
        Ok(())
    }
);
