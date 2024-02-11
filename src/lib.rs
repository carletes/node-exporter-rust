use std::fmt::Display;
use std::sync::Arc;

use anyhow::anyhow;
use parking_lot::RwLock;
use procfs::{CpuInfo, Current, CurrentSI, KernelStats, LoadAverage, Meminfo};
use prometheus::{Encoder, TextEncoder};
use static_init::dynamic;

mod metrics;

pub type Result<T> = anyhow::Result<T>;

trait SystemState {
    fn boot_time(&self) -> u64;
    fn context_switches(&self) -> u64;
    fn load15(&self) -> f32;
    fn load1(&self) -> f32;
    fn load5(&self) -> f32;
    fn mem_active_anon(&self) -> Option<u64>;
    fn mem_active_file(&self) -> Option<u64>;
    fn mem_active(&self) -> u64;
    fn mem_anon_huge_pages(&self) -> Option<u64>;
    fn mem_anon_pages(&self) -> Option<u64>;
    fn mem_bounce(&self) -> Option<u64>;
    fn mem_buffers(&self) -> u64;
    fn mem_cached(&self) -> u64;
    fn mem_cma_free(&self) -> Option<u64>;
    fn mem_cma_total(&self) -> Option<u64>;
    fn mem_commit_limit(&self) -> Option<u64>;
    fn mem_committed_as(&self) -> u64;
    fn mem_direct_map_1g(&self) -> Option<u64>;
    fn mem_direct_map_2m(&self) -> Option<u64>;
    fn mem_direct_map_4k(&self) -> Option<u64>;
    fn mem_direct_map_4m(&self) -> Option<u64>;
    fn mem_dirty(&self) -> u64;
    fn mem_file_huge_pages(&self) -> Option<u64>;
    fn mem_file_pmd_mapped(&self) -> Option<u64>;
    fn mem_hugepages_free(&self) -> Option<u64>;
    fn mem_hugepagesize(&self) -> Option<u64>;
    fn mem_hugepages_rsvd(&self) -> Option<u64>;
    fn mem_hugepages_surp(&self) -> Option<u64>;
    fn mem_hugepages_total(&self) -> Option<u64>;
    fn mem_hugetlb(&self) -> Option<u64>;
    fn mem_inactive_anon(&self) -> Option<u64>;
    fn mem_inactive_file(&self) -> Option<u64>;
    fn mem_inactive(&self) -> u64;
    fn mem_kernel_stack(&self) -> Option<u64>;
    fn mem_k_reclaimable(&self) -> Option<u64>;
    fn mem_mapped(&self) -> u64;
    fn mem_mem_available(&self) -> Option<u64>;
    fn mem_mem_free(&self) -> u64;
    fn mem_mem_total(&self) -> u64;
    fn mem_mlocked(&self) -> Option<u64>;
    fn mem_nfs_unstable(&self) -> Option<u64>;
    fn mem_page_tables(&self) -> Option<u64>;
    fn mem_per_cpu(&self) -> Option<u64>;
    fn mem_secondary_page_tables(&self) -> Option<u64>;
    fn mem_shmem_hugepages(&self) -> Option<u64>;
    fn mem_shmem_pmd_mapped(&self) -> Option<u64>;
    fn mem_shmem(&self) -> Option<u64>;
    fn mem_slab(&self) -> u64;
    fn mem_s_reclaimable(&self) -> Option<u64>;
    fn mem_s_unreclaim(&self) -> Option<u64>;
    fn mem_swap_cached(&self) -> u64;
    fn mem_swap_free(&self) -> u64;
    fn mem_swap_total(&self) -> u64;
    fn mem_unevictable(&self) -> Option<u64>;
    fn mem_vmalloc_chunk(&self) -> u64;
    fn mem_vmalloc_total(&self) -> u64;
    fn mem_vmalloc_used(&self) -> u64;
    fn mem_writeback(&self) -> u64;
    fn mem_writeback_tmp(&self) -> Option<u64>;
    fn mem_z_swapped(&self) -> Option<u64>;
    fn mem_z_swap(&self) -> Option<u64>;
    fn procs_blocked(&self) -> Option<u32>;
    fn procs_running(&self) -> Option<u32>;
}

struct CurrentSystemState {
    cpu_info: CpuInfo,
    kernel_stats: KernelStats,
    load_average: LoadAverage,
    meminfo: Meminfo,
}

impl CurrentSystemState {
    fn new() -> Result<Self> {
        Ok(CurrentSystemState {
            cpu_info: CpuInfo::current()?,
            kernel_stats: KernelStats::current()?,
            load_average: LoadAverage::current()?,
            meminfo: Meminfo::current()?,
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

    fn load1(&self) -> f32 {
        self.load_average.one
    }

    fn load15(&self) -> f32 {
        self.load_average.fifteen
    }

    fn load5(&self) -> f32 {
        self.load_average.five
    }

    fn mem_active(&self) -> u64 {
        self.meminfo.active
    }

    fn mem_active_anon(&self) -> Option<u64> {
        self.meminfo.active_anon
    }

    fn mem_active_file(&self) -> Option<u64> {
        self.meminfo.active_file
    }

    fn mem_anon_huge_pages(&self) -> Option<u64> {
        self.meminfo.anon_hugepages
    }

    fn mem_anon_pages(&self) -> Option<u64> {
        self.meminfo.anon_pages
    }

    fn mem_bounce(&self) -> Option<u64> {
        self.meminfo.bounce
    }

    fn mem_buffers(&self) -> u64 {
        self.meminfo.buffers
    }

    fn mem_cached(&self) -> u64 {
        self.meminfo.cached
    }

    fn mem_cma_free(&self) -> Option<u64> {
        self.meminfo.cma_free
    }

    fn mem_cma_total(&self) -> Option<u64> {
        self.meminfo.cma_total
    }

    fn mem_commit_limit(&self) -> Option<u64> {
        self.meminfo.commit_limit
    }

    fn mem_committed_as(&self) -> u64 {
        self.meminfo.committed_as
    }

    fn mem_direct_map_1g(&self) -> Option<u64> {
        self.meminfo.direct_map_1G
    }

    fn mem_direct_map_2m(&self) -> Option<u64> {
        self.meminfo.direct_map_2M
    }

    fn mem_direct_map_4m(&self) -> Option<u64> {
        self.meminfo.direct_map_4M
    }

    fn mem_direct_map_4k(&self) -> Option<u64> {
        self.meminfo.direct_map_4k
    }

    fn mem_file_huge_pages(&self) -> Option<u64> {
        self.meminfo.file_huge_pages
    }

    fn mem_file_pmd_mapped(&self) -> Option<u64> {
        self.meminfo.file_pmd_mapped
    }

    fn mem_hugepages_free(&self) -> Option<u64> {
        self.meminfo.hugepages_free
    }

    fn mem_hugepages_rsvd(&self) -> Option<u64> {
        self.meminfo.hugepages_rsvd
    }

    fn mem_hugepages_surp(&self) -> Option<u64> {
        self.meminfo.hugepages_surp
    }

    fn mem_hugepages_total(&self) -> Option<u64> {
        self.meminfo.hugepages_total
    }

    fn procs_blocked(&self) -> Option<u32> {
        self.kernel_stats.procs_blocked
    }

    fn procs_running(&self) -> Option<u32> {
        self.kernel_stats.procs_running
    }

    fn mem_dirty(&self) -> u64 {
        self.meminfo.dirty
    }

    fn mem_hugepagesize(&self) -> Option<u64> {
        self.meminfo.hugepagesize
    }

    fn mem_hugetlb(&self) -> Option<u64> {
        self.meminfo.hugetlb
    }

    fn mem_inactive_anon(&self) -> Option<u64> {
        self.meminfo.inactive_anon
    }

    fn mem_inactive(&self) -> u64 {
        self.meminfo.inactive
    }

    fn mem_inactive_file(&self) -> Option<u64> {
        self.meminfo.inactive_file
    }

    fn mem_kernel_stack(&self) -> Option<u64> {
        self.meminfo.kernel_stack
    }

    fn mem_k_reclaimable(&self) -> Option<u64> {
        self.meminfo.k_reclaimable
    }

    fn mem_mapped(&self) -> u64 {
        self.meminfo.mapped
    }

    fn mem_mem_available(&self) -> Option<u64> {
        self.meminfo.mem_available
    }

    fn mem_mem_free(&self) -> u64 {
        self.meminfo.mem_free
    }

    fn mem_mem_total(&self) -> u64 {
        self.meminfo.mem_total
    }

    fn mem_mlocked(&self) -> Option<u64> {
        self.meminfo.mlocked
    }

    fn mem_nfs_unstable(&self) -> Option<u64> {
        self.meminfo.nfs_unstable
    }

    fn mem_page_tables(&self) -> Option<u64> {
        self.meminfo.page_tables
    }

    fn mem_per_cpu(&self) -> Option<u64> {
        self.meminfo.per_cpu
    }

    fn mem_secondary_page_tables(&self) -> Option<u64> {
        self.meminfo.secondary_page_tables
    }

    fn mem_shmem(&self) -> Option<u64> {
        self.meminfo.shmem
    }

    fn mem_shmem_hugepages(&self) -> Option<u64> {
        self.meminfo.shmem_hugepages
    }

    fn mem_shmem_pmd_mapped(&self) -> Option<u64> {
        self.meminfo.shmem_pmd_mapped
    }

    fn mem_slab(&self) -> u64 {
        self.meminfo.slab
    }

    fn mem_s_reclaimable(&self) -> Option<u64> {
        self.meminfo.s_reclaimable
    }

    fn mem_s_unreclaim(&self) -> Option<u64> {
        self.meminfo.s_unreclaim
    }

    fn mem_swap_cached(&self) -> u64 {
        self.meminfo.swap_cached
    }

    fn mem_swap_free(&self) -> u64 {
        self.meminfo.swap_free
    }

    fn mem_swap_total(&self) -> u64 {
        self.meminfo.swap_total
    }

    fn mem_unevictable(&self) -> Option<u64> {
        self.meminfo.unevictable
    }

    fn mem_vmalloc_chunk(&self) -> u64 {
        self.meminfo.vmalloc_chunk
    }

    fn mem_vmalloc_total(&self) -> u64 {
        self.meminfo.vmalloc_total
    }

    fn mem_vmalloc_used(&self) -> u64 {
        self.meminfo.vmalloc_used
    }

    fn mem_writeback(&self) -> u64 {
        self.meminfo.writeback
    }

    fn mem_writeback_tmp(&self) -> Option<u64> {
        self.meminfo.writeback_tmp
    }

    fn mem_z_swap(&self) -> Option<u64> {
        self.meminfo.z_swap
    }

    fn mem_z_swapped(&self) -> Option<u64> {
        self.meminfo.z_swapped
    }
}

enum Metric {
    Gauge(prometheus::Gauge),
    GaugeVec(prometheus::GaugeVec),
    IntCounter(prometheus::IntCounter),
    IntCounterVec(prometheus::IntCounterVec),
    IntGauge(prometheus::IntGauge),
}

impl TryInto<prometheus::Gauge> for &Metric {
    type Error = anyhow::Error;

    fn try_into(self) -> std::result::Result<prometheus::Gauge, Self::Error> {
        match self {
            Metric::Gauge(g) => Ok(g.clone()),
            _ => Err(anyhow!("Nope")),
        }
    }
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
    metrics: Vec<MetricUpdate>,
}

impl UpdateRegistryCore {
    fn register(&mut self, m: MetricUpdate) {
        self.metrics.push(m);
    }
}

#[derive(Default)]
struct UpdateRegistry {
    r: Arc<RwLock<UpdateRegistryCore>>,
}

impl UpdateRegistry {
    fn register(&self, m: MetricUpdate) {
        self.r.write().register(m);
    }

    fn dump(&self, state: &dyn SystemState) -> Result<(String, Vec<String>)> {
        let mut errors: Vec<String> = Vec::new();

        for m in &self.r.read().metrics {
            let _ = m.update(state).map_err(|err| {
                errors.push(format!("{}: {}", m.name, err));
                err
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
static UPDATE_REGISTRY: UpdateRegistry = UpdateRegistry::default();

pub fn dump() -> Result<(String, Vec<String>)> {
    let state = CurrentSystemState::new()?;
    dump_with_state(&state)
}

fn dump_with_state(state: &dyn SystemState) -> Result<(String, Vec<String>)> {
    UPDATE_REGISTRY.dump(state)
}

#[macro_export]
macro_rules! register_metric {
    ($ctor: ident, $type: ident, $help: literal, $update: expr) => {
        #[static_init::constructor]
        #[allow(non_snake_case)]
        extern "C" fn $ctor() {
            let m = prometheus::$type::with_opts(prometheus::opts!(stringify!($ctor), $help))
                .expect("Error registering metric");
            let _ = prometheus::register(Box::new(m.clone()));

            let update = $crate::MetricUpdate {
                name: stringify!($ctor).to_string(),
                metric: $crate::Metric::$type(m),
                update: Box::new($update),
            };
            $crate::UPDATE_REGISTRY.register(update);
        }
    };
}

#[macro_export]
macro_rules! register_metric_vec {
    ($ctor: ident, $type: ident, $help: literal, $labels: expr, $update: expr) => {
        #[static_init::constructor]
        #[allow(non_snake_case)]
        extern "C" fn $ctor() {
            let m: $type =
                prometheus::$type::new(prometheus::opts!(stringify!($ctor), $help), $labels)
                    .expect("Error registering metric ");
            let _ = prometheus::register(Box::new(m.clone()));

            let update = $crate::MetricUpdate {
                name: stringify!($ctor).to_string(),
                metric: $crate::Metric::$type(m),
                update: Box::new($update),
            };
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
        pub load15: f32,
        pub load1: f32,
        pub load5: f32,
        pub mem_active_anon: Option<u64>,
        pub mem_active_file: Option<u64>,
        pub mem_active: u64,
        pub mem_anon_huge_pages: Option<u64>,
        pub mem_anon_pages: Option<u64>,
        pub mem_bounce: Option<u64>,
        pub mem_buffers: u64,
        pub mem_cached: u64,
        pub mem_cma_free: Option<u64>,
        pub mem_cma_total: Option<u64>,
        pub mem_commit_limit: Option<u64>,
        pub mem_committed_as: u64,
        pub mem_direct_map_1g: Option<u64>,
        pub mem_direct_map_2m: Option<u64>,
        pub mem_direct_map_4k: Option<u64>,
        pub mem_direct_map_4m: Option<u64>,
        pub mem_dirty: u64,
        pub mem_file_huge_pages: Option<u64>,
        pub mem_file_pmd_mapped: Option<u64>,
        pub mem_hugepages_free: Option<u64>,
        pub mem_hugepagesize: Option<u64>,
        pub mem_hugepages_rsvd: Option<u64>,
        pub mem_hugepages_surp: Option<u64>,
        pub mem_hugepages_total: Option<u64>,
        pub mem_hugetlb: Option<u64>,
        pub mem_inactive_anon: Option<u64>,
        pub mem_inactive_file: Option<u64>,
        pub mem_inactive: u64,
        pub mem_kernel_stack: Option<u64>,
        pub mem_k_reclaimable: Option<u64>,
        pub mem_mapped: u64,
        pub mem_mem_available: Option<u64>,
        pub mem_mem_free: u64,
        pub mem_mem_total: u64,
        pub mem_mlocked: Option<u64>,
        pub mem_nfs_unstable: Option<u64>,
        pub mem_page_tables: Option<u64>,
        pub mem_per_cpu: Option<u64>,
        pub mem_secondary_page_tables: Option<u64>,
        pub mem_shmem_hugepages: Option<u64>,
        pub mem_shmem: Option<u64>,
        pub mem_shmem_pmd_mapped: Option<u64>,
        pub mem_slab: u64,
        pub mem_s_reclaimable: Option<u64>,
        pub mem_s_unreclaim: Option<u64>,
        pub mem_swap_cached: u64,
        pub mem_swap_free: u64,
        pub mem_swap_total: u64,
        pub mem_unevictable: Option<u64>,
        pub mem_vmalloc_chunk: u64,
        pub mem_vmalloc_total: u64,
        pub mem_vmalloc_used: u64,
        pub mem_writeback_tmp: Option<u64>,
        pub mem_writeback: u64,
        pub mem_z_swap: Option<u64>,
        pub mem_z_swapped: Option<u64>,
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

        fn load1(&self) -> f32 {
            self.load1
        }

        fn load15(&self) -> f32 {
            self.load15
        }

        fn load5(&self) -> f32 {
            self.load5
        }

        fn mem_active(&self) -> u64 {
            self.mem_active
        }

        fn mem_active_anon(&self) -> Option<u64> {
            self.mem_active_anon
        }

        fn mem_active_file(&self) -> Option<u64> {
            self.mem_active_file
        }

        fn mem_anon_huge_pages(&self) -> Option<u64> {
            self.mem_anon_huge_pages
        }

        fn mem_anon_pages(&self) -> Option<u64> {
            self.mem_anon_pages
        }

        fn mem_bounce(&self) -> Option<u64> {
            self.mem_bounce
        }

        fn mem_buffers(&self) -> u64 {
            self.mem_buffers
        }

        fn mem_cached(&self) -> u64 {
            self.mem_cached
        }

        fn mem_cma_free(&self) -> Option<u64> {
            self.mem_cma_free
        }

        fn mem_cma_total(&self) -> Option<u64> {
            self.mem_cma_total
        }

        fn mem_commit_limit(&self) -> Option<u64> {
            self.mem_commit_limit
        }

        fn mem_committed_as(&self) -> u64 {
            self.mem_committed_as
        }

        fn mem_direct_map_1g(&self) -> Option<u64> {
            self.mem_direct_map_1g
        }

        fn mem_direct_map_2m(&self) -> Option<u64> {
            self.mem_direct_map_2m
        }

        fn mem_direct_map_4m(&self) -> Option<u64> {
            self.mem_direct_map_4m
        }

        fn mem_direct_map_4k(&self) -> Option<u64> {
            self.mem_direct_map_4k
        }

        fn mem_file_huge_pages(&self) -> Option<u64> {
            self.mem_file_huge_pages
        }

        fn mem_file_pmd_mapped(&self) -> Option<u64> {
            self.mem_file_pmd_mapped
        }

        fn mem_hugepages_free(&self) -> Option<u64> {
            self.mem_hugepages_free
        }

        fn mem_hugepages_rsvd(&self) -> Option<u64> {
            self.mem_hugepages_rsvd
        }

        fn mem_hugepages_surp(&self) -> Option<u64> {
            self.mem_hugepages_surp
        }

        fn mem_hugepages_total(&self) -> Option<u64> {
            self.mem_hugepages_total
        }

        fn procs_blocked(&self) -> Option<u32> {
            self.procs_blocked
        }

        fn procs_running(&self) -> Option<u32> {
            self.procs_running
        }

        fn mem_dirty(&self) -> u64 {
            self.mem_dirty
        }

        fn mem_hugepagesize(&self) -> Option<u64> {
            self.mem_hugepagesize
        }

        fn mem_hugetlb(&self) -> Option<u64> {
            self.mem_hugetlb
        }

        fn mem_inactive_anon(&self) -> Option<u64> {
            self.mem_inactive_anon
        }

        fn mem_inactive(&self) -> u64 {
            self.mem_inactive
        }

        fn mem_inactive_file(&self) -> Option<u64> {
            self.mem_inactive_file
        }

        fn mem_kernel_stack(&self) -> Option<u64> {
            self.mem_kernel_stack
        }

        fn mem_k_reclaimable(&self) -> Option<u64> {
            self.mem_k_reclaimable
        }

        fn mem_mapped(&self) -> u64 {
            self.mem_mapped
        }

        fn mem_mem_available(&self) -> Option<u64> {
            self.mem_mem_available
        }

        fn mem_mem_free(&self) -> u64 {
            self.mem_mem_free
        }

        fn mem_mem_total(&self) -> u64 {
            self.mem_mem_total
        }

        fn mem_mlocked(&self) -> Option<u64> {
            self.mem_mlocked
        }

        fn mem_nfs_unstable(&self) -> Option<u64> {
            self.mem_nfs_unstable
        }

        fn mem_page_tables(&self) -> Option<u64> {
            self.mem_page_tables
        }

        fn mem_per_cpu(&self) -> Option<u64> {
            self.mem_per_cpu
        }

        fn mem_secondary_page_tables(&self) -> Option<u64> {
            self.mem_secondary_page_tables
        }

        fn mem_shmem(&self) -> Option<u64> {
            self.mem_shmem
        }

        fn mem_shmem_hugepages(&self) -> Option<u64> {
            self.mem_shmem_hugepages
        }

        fn mem_shmem_pmd_mapped(&self) -> Option<u64> {
            self.mem_shmem_pmd_mapped
        }

        fn mem_slab(&self) -> u64 {
            self.mem_slab
        }

        fn mem_s_reclaimable(&self) -> Option<u64> {
            self.mem_s_reclaimable
        }

        fn mem_s_unreclaim(&self) -> Option<u64> {
            self.mem_s_unreclaim
        }

        fn mem_swap_cached(&self) -> u64 {
            self.mem_swap_cached
        }

        fn mem_swap_free(&self) -> u64 {
            self.mem_swap_free
        }

        fn mem_swap_total(&self) -> u64 {
            self.mem_swap_total
        }

        fn mem_unevictable(&self) -> Option<u64> {
            self.mem_unevictable
        }

        fn mem_vmalloc_chunk(&self) -> u64 {
            self.mem_vmalloc_chunk
        }

        fn mem_vmalloc_total(&self) -> u64 {
            self.mem_vmalloc_total
        }

        fn mem_vmalloc_used(&self) -> u64 {
            self.mem_vmalloc_used
        }

        fn mem_writeback(&self) -> u64 {
            self.mem_writeback
        }

        fn mem_writeback_tmp(&self) -> Option<u64> {
            self.mem_writeback_tmp
        }

        fn mem_z_swap(&self) -> Option<u64> {
            self.mem_z_swap
        }

        fn mem_z_swapped(&self) -> Option<u64> {
            self.mem_z_swapped
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
