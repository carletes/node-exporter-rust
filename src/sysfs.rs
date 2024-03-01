use std::collections::HashSet;
use std::{fs, path::Path};

use procfs::{CpuInfo, Current, ProcResult};

/// CPU thread information and statistics.
#[derive(Default)]
pub struct CpuStat {
    /// Core ID of this CPU thread.
    pub core_id: u64,

    /// Maximum CPU thread frequency in hertz.
    pub max_freq: u64,

    /// Minimum CPU thread frequency in hertz.
    pub min_freq: u64,

    /// Number of thermal throttles for this CPU thread's core.
    pub num_core_throttles: u64,

    /// Number of thermal throttles for this CPU thread's package.
    pub num_package_throttles: u64,

    /// Physical package ID of this CPU thread.
    pub physical_package_id: u64,

    /// Available CPU frequency governors.
    pub scaling_available_governors: HashSet<String>,

    /// Current scaled CPU thread frequency in hertz.
    pub scaling_cur_freq: u64,

    /// Current enabled CPU frequency governor.
    pub scaling_governor: String,

    /// Maximum scaled CPU thread frequency in hertz.
    pub scaling_max_freq: u64,

    /// Minimum scaled CPU thread frequency in hertz.
    pub scaling_min_freq: u64,
}

impl CpuStat {
    fn u64_from_file(fname: &Path) -> ProcResult<u64> {
        let res = fs::read_to_string(fname)?.trim().parse::<u64>()?;
        Ok(res)
    }

    fn from_path(p: &Path) -> ProcResult<Self> {
        let mut governors: HashSet<String> = HashSet::new();
        governors.extend(
            fs::read_to_string(p.join("cpufreq/scaling_available_governors"))?
                .trim()
                .split(' ')
                .map(|g| g.into()),
        );
        Ok(CpuStat {
            core_id: Self::u64_from_file(&p.join("topology/core_id"))?,
            max_freq: Self::u64_from_file(&p.join("cpufreq/cpuinfo_max_freq"))? * 1_000,
            min_freq: Self::u64_from_file(&p.join("cpufreq/cpuinfo_min_freq"))? * 1_000,
            num_core_throttles: Self::u64_from_file(
                &p.join("thermal_throttle/core_throttle_count"),
            )?,
            num_package_throttles: Self::u64_from_file(
                &p.join("thermal_throttle/package_throttle_count"),
            )?,
            physical_package_id: fs::read_to_string(p.join("topology/physical_package_id"))?
                .trim()
                .parse::<u64>()?,
            scaling_available_governors: governors,
            scaling_cur_freq: Self::u64_from_file(&p.join("cpufreq/scaling_cur_freq"))? * 1_000,
            scaling_governor: fs::read_to_string(p.join("cpufreq/scaling_governor"))?
                .trim()
                .into(),
            scaling_max_freq: Self::u64_from_file(&p.join("cpufreq/scaling_max_freq"))? * 1_000,
            scaling_min_freq: Self::u64_from_file(&p.join("cpufreq/scaling_min_freq"))? * 1_000,
        })
    }
}

/// A list of CPU freqency stats, one for each CPU thread.
#[derive(Default)]
pub struct CpuStats {
    pub threads: Vec<CpuStat>,
}

impl CpuStats {
    pub fn current() -> ProcResult<Self> {
        let n_cpus = CpuInfo::current()?.num_cores();
        let mut stats: Vec<CpuStat> = Vec::with_capacity(n_cpus);
        for i in 0..n_cpus {
            let p = format!("/sys/devices/system/cpu/cpu{}", i);
            stats.push(CpuStat::from_path(Path::new(&p))?);
        }

        Ok(CpuStats { threads: stats })
    }
}

#[cfg(test)]
mod tests_cpu_freq {
    use super::CpuStats;
    use super::ProcResult;

    #[test]
    fn stats_for_first_cpu() -> ProcResult<()> {
        let cpus = CpuStats::current()?.threads;
        assert!(cpus.len() > 0);

        let cpu0 = &cpus[0];
        assert!(cpu0.min_freq > 0);
        assert!(cpu0.max_freq >= cpu0.min_freq);

        assert!(cpu0.scaling_cur_freq >= cpu0.min_freq);
        assert!(cpu0.scaling_cur_freq <= cpu0.max_freq);

        assert!(cpu0.scaling_min_freq >= cpu0.min_freq);
        assert!(cpu0.scaling_min_freq <= cpu0.scaling_max_freq);
        assert!(cpu0.scaling_max_freq <= cpu0.max_freq);

        assert!(cpu0.scaling_available_governors.len() > 0);
        assert!(cpu0
            .scaling_available_governors
            .contains(&cpu0.scaling_governor));

        Ok(())
    }
}
