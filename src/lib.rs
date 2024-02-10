use std::fmt::Display;
use std::sync::Arc;

use anyhow::anyhow;
use parking_lot::RwLock;
use procfs::{CpuInfo, Current, CurrentSI, KernelStats};
use prometheus::{Encoder, TextEncoder};
use static_init::dynamic;

mod metrics;

pub type Result<T> = anyhow::Result<T>;

trait SystemState {
    fn boot_time(&self) -> u64;
    fn context_switches(&self) -> u64;
    fn procs_blocked(&self) -> Option<u32>;
    fn procs_running(&self) -> Option<u32>;
}

struct CurrentSystemState {
    kernel_stats: KernelStats,
    cpu_info: CpuInfo,
}

impl CurrentSystemState {
    fn new() -> Result<Self> {
        Ok(CurrentSystemState {
            cpu_info: CpuInfo::current()?,
            kernel_stats: KernelStats::current()?,
        })
    }
}

impl SystemState for CurrentSystemState {
    fn boot_time(&self) -> u64 {
        self.kernel_stats.btime
    }

    fn context_switches(&self) -> u64 {
        self.kernel_stats.ctxt
    }

    fn procs_blocked(&self) -> Option<u32> {
        self.kernel_stats.procs_blocked
    }

    fn procs_running(&self) -> Option<u32> {
        self.kernel_stats.procs_running
    }
}

enum Metric {
    GaugeVec(prometheus::GaugeVec),
    IntCounter(prometheus::IntCounter),
    IntCounterVec(prometheus::IntCounterVec),
    IntGauge(prometheus::IntGauge),
}

impl TryInto<prometheus::GaugeVec> for &Metric {
    type Error = anyhow::Error;

    fn try_into(self) -> std::result::Result<prometheus::GaugeVec, Self::Error> {
        match self {
            Metric::GaugeVec(v) => Ok(v.clone()),
            _ => Err(anyhow!("Nope")),
        }
    }
}

impl TryInto<prometheus::IntCounter> for &Metric {
    type Error = anyhow::Error;

    fn try_into(self) -> std::result::Result<prometheus::IntCounter, Self::Error> {
        match self {
            Metric::IntCounter(c) => Ok(c.clone()),
            _ => Err(anyhow!("Nope")),
        }
    }
}

impl TryInto<prometheus::IntCounterVec> for &Metric {
    type Error = anyhow::Error;

    fn try_into(self) -> std::result::Result<prometheus::IntCounterVec, Self::Error> {
        match self {
            Metric::IntCounterVec(c) => Ok(c.clone()),
            _ => Err(anyhow!("Nope")),
        }
    }
}

impl TryInto<prometheus::IntGauge> for &Metric {
    type Error = anyhow::Error;

    fn try_into(self) -> std::result::Result<prometheus::IntGauge, Self::Error> {
        match self {
            Metric::IntGauge(g) => Ok(g.clone()),
            _ => Err(anyhow!("Nope")),
        }
    }
}

struct MetricUpdate {
    name: String,
    metric: Metric,
    update: Box<dyn Fn(&dyn SystemState, &Metric) -> Result<()> + Send + Sync>,
}

impl MetricUpdate {
    fn update(&self, state: &dyn SystemState) -> Result<()> {
        (self.update)(state, &self.metric)
    }
}

impl Display for MetricUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Default)]
struct UpdateRegistryCore {
    metrics: Vec<Box<MetricUpdate>>,
}

impl UpdateRegistryCore {
    fn register(&mut self, m: Box<MetricUpdate>) -> () {
        self.metrics.push(m);
    }
}

#[derive(Default)]
struct UpdateRegistry {
    r: Arc<RwLock<UpdateRegistryCore>>,
}

impl UpdateRegistry {
    fn register(&self, m: Box<MetricUpdate>) -> () {
        self.r.write().register(m);
    }

    fn dump(&self, state: &dyn SystemState) -> Result<(String, Vec<String>)> {
        let mut errors: Vec<String> = Vec::new();

        for m in &self.r.read().metrics {
            let _ = m.update(state).or_else(|err| {
                errors.push(format!("{}: {}", m.name, err.to_string()));
                Err(err)
            });
        }

        let mut buffer = vec![];
        let encoder = TextEncoder::new();
        let metric_families = prometheus::gather();
        encoder.encode(&metric_families, &mut buffer)?;
        Ok((String::from_utf8(buffer)?, errors))
    }
}

#[dynamic]
static UPDATE_REGISTRY: UpdateRegistry = {
    let r = UpdateRegistry::default();
    r
};

pub fn dump() -> Result<(String, Vec<String>)> {
    let state = CurrentSystemState::new()?;
    dump_with_state(&state)
}

fn dump_with_state(state: &dyn SystemState) -> Result<(String, Vec<String>)> {
    Ok(UPDATE_REGISTRY.dump(state)?)
}

#[macro_export]
macro_rules! register_metric {
    ($ctor: ident, $type: ident, $help: literal, $update: expr) => {
        #[static_init::constructor]
        extern "C" fn $ctor() {
            let m = prometheus::$type::with_opts(prometheus::opts!(stringify!($ctor), $help))
                .expect("Error registering metric");
            let _ = prometheus::register(Box::new(m.clone()));

            let update = Box::new($crate::MetricUpdate {
                name: stringify!($ctor).to_string(),
                metric: $crate::Metric::$type(m),
                update: Box::new($update),
            });
            $crate::UPDATE_REGISTRY.register(update);
        }
    };
}

#[macro_export]
macro_rules! register_metric_vec {
    ($ctor: ident, $type: ident, $help: literal, $labels: expr, $update: expr) => {
        #[static_init::constructor]
        extern "C" fn $ctor() {
            let m: $type =
                prometheus::$type::new(prometheus::opts!(stringify!($ctor), $help), $labels)
                    .expect("Error registering metric ");
            let _ = prometheus::register(Box::new(m.clone()));

            let update = Box::new($crate::MetricUpdate {
                name: stringify!($ctor).to_string(),
                metric: $crate::Metric::$type(m),
                update: Box::new($update),
            });
            $crate::UPDATE_REGISTRY.register(update);
        }
    };
}

#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;
    use parking_lot::{const_mutex, Mutex};

    use super::SystemState;

    #[derive(Default)]
    pub struct MockSystemState {
        pub boot_time: u64,
        pub context_switches: u64,
        pub procs_blocked: Option<u32>,
        pub procs_running: Option<u32>,
    }

    impl SystemState for MockSystemState {
        fn boot_time(&self) -> u64 {
            self.boot_time
        }

        fn context_switches(&self) -> u64 {
            self.context_switches
        }

        fn procs_blocked(&self) -> Option<u32> {
            self.procs_blocked
        }

        fn procs_running(&self) -> Option<u32> {
            self.procs_running
        }
    }

    lazy_static! {
        static ref LOCK: Mutex<u8> = const_mutex(0);
    }

    pub fn dump_with_state(state: &dyn SystemState) -> super::Result<(String, Vec<String>)> {
        let _unused = LOCK.lock();
        super::dump_with_state(state)
    }
}
