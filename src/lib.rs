use std::sync::Arc;

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

struct MetricUpdate {
    update: Box<dyn Fn(&dyn SystemState) -> Result<()> + Send + Sync>,
}

impl MetricUpdate {
    fn update(&self, state: &dyn SystemState) -> Result<()> {
        (self.update)(state)?;
        Ok(())
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

    fn dump(&self, state: &dyn SystemState) -> Result<String> {
        for m in &self.r.read().metrics {
            // XXX Collect errors instead of returning a failure after the first error.
            m.update(state)?;
        }

        let mut buffer = vec![];
        let encoder = TextEncoder::new();
        let metric_families = prometheus::gather();
        encoder.encode(&metric_families, &mut buffer)?;
        Ok(String::from_utf8(buffer)?)
    }
}

#[dynamic]
static UPDATE_REGISTRY: UpdateRegistry = {
    let r = UpdateRegistry::default();
    r
};

pub fn dump() -> Result<String> {
    let state = CurrentSystemState::new()?;
    Ok(UPDATE_REGISTRY.dump(&state)?)
}

#[macro_export]
macro_rules! register_metric {
    ($ref: ident, $type: ident, $ctor: ident, $help: literal, $update: expr) => {
        #[dynamic]
        static $ref: $type = {
            let m: $type =
                prometheus::$type::with_opts(prometheus::opts!(stringify!($ctor), $help))
                    .expect("Error registering metric");
            let _ = prometheus::register(Box::new(m.clone()));
            m
        };

        #[constructor]
        extern "C" fn $ctor() {
            let update = Box::new($crate::MetricUpdate {
                update: Box::new($update),
            });
            $crate::UPDATE_REGISTRY.register(update);
        }
    };
}

#[macro_export]
macro_rules! register_metric_vec {
    ($ref: ident, $type: ident, $ctor: ident, $help: literal, $labels: expr, $update: expr) => {
        #[dynamic]
        static $ref: $type = {
            let m: $type =
                prometheus::$type::new(prometheus::opts!(stringify!($ctor), $help), $labels)
                    .expect("Error registering metric ");
            let _ = prometheus::register(Box::new(m.clone()));
            m
        };

        #[constructor]
        extern "C" fn $ctor() {
            let update = Box::new($crate::MetricUpdate {
                update: Box::new($update),
            });
            $crate::UPDATE_REGISTRY.register(update);
        }
    };
}
