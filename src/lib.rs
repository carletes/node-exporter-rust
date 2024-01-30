use std::sync::Arc;

use parking_lot::RwLock;
use procfs::KernelStats;
use prometheus::{Encoder, TextEncoder};
use static_init::dynamic;

mod metrics;

pub struct SystemState {
    pub kernel_stats: KernelStats,
}

struct MetricUpdate {
    update: Box<dyn Fn(&SystemState) -> () + Send + Sync>,
}

impl MetricUpdate {
    fn update(&self, state: &SystemState) -> () {
        (self.update)(state);
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

    fn dump(&self, state: &SystemState) -> String {
        for m in &self.r.read().metrics {
            m.update(state);
        }

        let mut buffer = vec![];
        let encoder = TextEncoder::new();
        let metric_families = prometheus::gather();
        encoder.encode(&metric_families, &mut buffer).unwrap();
        String::from_utf8(buffer).unwrap()
    }
}

#[dynamic]
static UPDATE_REGISTRY: UpdateRegistry = {
    let r = UpdateRegistry::default();
    r
};

pub fn dump(state: &SystemState) -> String {
    UPDATE_REGISTRY.dump(state)
}

#[macro_export]
macro_rules! register_metric {
    ($ref: ident, $type: ident, $ctor: ident, $help: literal, $update: expr) => {
        #[dynamic]
        static $ref: $type = {
            let m: $type =
                prometheus::$type::with_opts(prometheus::opts!(stringify!($ctor), $help)).unwrap();
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
                    .unwrap();
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
