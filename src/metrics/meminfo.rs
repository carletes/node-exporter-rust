use crate::register_metric;

use prometheus::IntGauge;

macro_rules! register_meminfo_metric {
    ($ctor: ident, $help: literal, $state_field: ident, $test_mod: ident) => {
        register_metric!($ctor, IntGauge, $help, |state, m| {
            let g: IntGauge = m.try_into()?;
            g.set(i64::try_from(state.$state_field())?);
            Ok(())
        });

        #[cfg(test)]
        mod $test_mod {
            use crate::tests::{dump_with_state, MockSystemState};
            use crate::Result;

            #[test]
            fn valid() -> Result<()> {
                let mut state = MockSystemState::default();
                state.$state_field = 3478810624;
                let (m, _) = dump_with_state(&state)?;
                assert!(
                    m.contains(concat!(stringify!($ctor), " 3478810624")),
                    "Metrics: <{}>",
                    m
                );
                Ok(())
            }

            #[test]
            fn too_large() -> Result<()> {
                let mut state = MockSystemState::default();
                state.$state_field = (i64::MAX as u64) + 1;
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

macro_rules! register_meminfo_optional_metric {
    ($ctor: ident, $help: literal, $state_field: ident, $test_mod: ident) => {
        register_metric!($ctor, IntGauge, $help, |state, m| {
            let g: IntGauge = m.try_into()?;
            match state.$state_field() {
                Some(bytes) => {
                    g.set(i64::try_from(bytes)?);
                }
                None => {}
            }
            Ok(())
        });

        #[cfg(test)]
        mod $test_mod {
            use crate::tests::{dump_with_state, MockSystemState};
            use crate::Result;

            #[test]
            fn valid() -> Result<()> {
                let mut state = MockSystemState::default();
                state.$state_field = Some(3478810624);
                let (m, _) = dump_with_state(&state)?;
                assert!(
                    m.contains(concat!(stringify!($ctor), " 3478810624")),
                    "Metrics: <{}>",
                    m
                );
                Ok(())
            }

            #[test]
            fn too_large() -> Result<()> {
                let mut state = MockSystemState::default();
                state.$state_field = Some((i64::MAX as u64) + 1);
                let (m, err) = dump_with_state(&state)?;
                assert!(
                    m.contains(concat!(stringify!($ctor), " ")),
                    "Metrics: <{}>",
                    m
                );
                assert!(
                    err.contains(
                        &concat!(
                            stringify!($ctor),
                            ": out of range integral type conversion attempted"
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

register_meminfo_metric!(
    node_memory_Active_bytes,
    "Memory that has been used more recently and usually not reclaimed unless absolutely necessary.",
    mem_active,
    tests_mem_active
);

register_meminfo_optional_metric!(
    node_memory_Active_anon_bytes,
    "Anonymous memory that has been used more recently and usually not swapped out.",
    mem_active_anon,
    tests_memory_active_anon
);

register_meminfo_optional_metric!(
    node_memory_Active_file_bytes,
    "Pagecache memory that has been used more recently and usually not reclaimed until needed.",
    mem_active_file,
    tests_mem_active_file
);

register_meminfo_optional_metric!(
    node_memory_AnonHugePages_bytes,
    "Non-file backed huge pages mapped into user-space page tables.",
    mem_anon_huge_pages,
    tests_mem_anon_huge_pages
);

register_meminfo_optional_metric!(
    node_memory_AnonPages_bytes,
    "Non-file backed pages mapped into user-space page tables.",
    mem_anon_pages,
    tests_mem_anon_pages
);

register_meminfo_optional_metric!(
    node_memory_Bounce_bytes,
    "Memory used for block device bounce buffers.",
    mem_bounce,
    tests_mem_bounce
);

register_meminfo_metric!(
    node_memory_Buffers_bytes,
    "Relatively temporary storage for raw disk blocks that shouldn’t get tremendously large (20MB or so).",
    mem_buffers,
    tests_mem_buffers
);

register_meminfo_metric!(
    node_memory_Cached_bytes,
    "In-memory cache for files read from the disk (the page cache). Doesn’t include SwapCached.",
    mem_cached,
    tests_mem_cached
);

register_meminfo_optional_metric!(
    node_memory_CmaFree_bytes,
    "Free CMA (Contiguous Memory Allocator) pages.",
    mem_cma_free,
    tests_mem_cma_free
);

register_meminfo_optional_metric!(
    node_memory_CmaTotal_bytes,
    "Total CMA (Contiguous Memory Allocator) pages.",
    mem_cma_total,
    tests_mem_cma_total
);

register_meminfo_optional_metric!(
    node_memory_CommitLimit_bytes,
    "Total amount of memory currently available to be allocated on the system.",
    mem_commit_limit,
    tests_mem_commit_limit
);

register_meminfo_metric!(
    node_memory_Committed_AS_bytes,
    "Amount of memory presently allocated on the system.",
    mem_committed_as,
    tests_mem_committed_as
);

register_meminfo_optional_metric!(
    node_memory_DirectMap1G_bytes,
    "Number of bytes of RAM linearly mapped by kernel in 1GB pages.",
    mem_direct_map_1g,
    tests_mem_direct_map_1g
);

register_meminfo_optional_metric!(
    node_memory_DirectMap2M_bytes,
    "Number of bytes of RAM linearly mapped by kernel in 2MB pages.",
    mem_direct_map_2m,
    tests_mem_direct_map_2m
);

register_meminfo_optional_metric!(
    node_memory_DirectMap4M_bytes,
    "Number of bytes of RAM linearly mapped by kernel in 4MB pages.",
    mem_direct_map_4m,
    tests_mem_direct_map_4m
);

register_meminfo_optional_metric!(
    node_memory_DirectMap4k_bytes,
    "Number of bytes of RAM linearly mapped by kernel in kB pages.",
    mem_direct_map_4k,
    tests_mem_direct_map_4k
);

register_meminfo_optional_metric!(
    node_memory_FileHugePages_bytes,
    "Memory consumed by page cache allocated with huge pages.",
    mem_file_huge_pages,
    tests_mem_file_huge_pages
);

register_meminfo_optional_metric!(
    node_memory_FilePmdMapped_bytes,
    "Mapped page cache in the userspace allocated with huge pages.",
    mem_file_pmd_mapped,
    tests_mem_file_pmd_mapped
);

register_meminfo_optional_metric!(
    node_memory_HugePages_Free,
    "Number of huge pages in the pool that are not yet allocated.",
    mem_hugepages_free,
    tests_mem_hugepages_free
);

register_meminfo_optional_metric!(
    node_memory_HugePages_Rsvd,
    "Number of huge pages for which a commitment to allocate from the pool has been made, but no allocation has yet been made.",
    mem_hugepages_rsvd,
    tests_mem_hugepages_rsvd
);

register_meminfo_optional_metric!(
    node_memory_HugePages_Surp,
    "Number of huge pages in the pool above the value in /proc/sys/vm/nr_hugepages.",
    mem_hugepages_surp,
    tests_mem_hugepages_surp
);

register_meminfo_optional_metric!(
    node_memory_HugePages_Total,
    "Size of the pool of huge pages.",
    mem_hugepages_total,
    tests_mem_hugepages_total
);

// Machine-generated metrics follow.

register_meminfo_metric!(
    node_memory_Dirty_bytes,
    "Memory which is waiting to get written back to the disk.",
    mem_dirty,
    tests_dirty
);

register_meminfo_optional_metric!(
    node_memory_Hugepagesize_bytes,
    "The size of huge pages.",
    mem_hugepagesize,
    tests_hugepagesize
);

register_meminfo_optional_metric!(
    node_memory_Hugetlb_bytes,
    "Total amount of memory consumed by huge pages of all sizes.",
    mem_hugetlb,
    tests_hugetlb
);

register_meminfo_optional_metric!(
    node_memory_Inactive_anon_bytes,
    "Anonymous memory that has not been used recently and can be swapped out.",
    mem_inactive_anon,
    tests_inactive_anon
);

register_meminfo_metric!(
    node_memory_Inactive_bytes,
    "Memory that has not been used recently and can be swapped out or reclaimed.",
    mem_inactive,
    tests_inactive
);

register_meminfo_optional_metric!(
    node_memory_Inactive_file_bytes,
    "Pagecache memory that can be reclaimed without huge performance impact.",
    mem_inactive_file,
    tests_inactive_file
);

register_meminfo_optional_metric!(
    node_memory_KernelStack_bytes,
    "Amount of memory allocated to kernel stacks.",
    mem_kernel_stack,
    tests_kernel_stack
);

register_meminfo_optional_metric!(
    node_memory_KReclaimable_bytes,
    "Kernel allocations that the kernel will attempt to reclaim under memory pressure.",
    mem_k_reclaimable,
    tests_k_reclaimable
);

register_meminfo_metric!(
    node_memory_Mapped_bytes,
    "Files which have been mapped into memory.",
    mem_mapped,
    tests_mapped
);

register_meminfo_optional_metric!(
    node_memory_MemAvailable_bytes,
    "Estimate of how much memory is available for starting new applications, without swapping.",
    mem_mem_available,
    tests_mem_available
);

register_meminfo_metric!(
    node_memory_MemFree_bytes,
    "Amount of physical memory not used by the system.",
    mem_mem_free,
    tests_mem_free
);

register_meminfo_metric!(
    node_memory_MemTotal_bytes,
    "Total usable RAM (i.e., physical RAM minus a few reserved bits and the kernel binary code).",
    mem_mem_total,
    tests_mem_total
);

register_meminfo_optional_metric!(
    node_memory_Mlocked_bytes,
    "Pages locked to memory using the mlock() system call.",
    mem_mlocked,
    tests_mlocked
);

register_meminfo_optional_metric!(
    node_memory_NFS_Unstable_bytes,
    "NFS pages sent to the server, but not yet committed to stable storage.",
    mem_nfs_unstable,
    tests_nfs_unstable
);

register_meminfo_optional_metric!(
    node_memory_PageTables_bytes,
    "Amount of memory dedicated to the lowest level of page tables.",
    mem_page_tables,
    tests_page_tables
);

register_meminfo_optional_metric!(
    node_memory_Percpu_bytes,
    "Memory allocated to the per-cpu alloctor used to back per-cpu allocations.",
    mem_per_cpu,
    tests_per_cpu
);

register_meminfo_optional_metric!(
    node_memory_SecPageTables_bytes,
    "Amount of memory allocated for seconary page tables.",
    mem_secondary_page_tables,
    tests_secondary_page_tables
);

register_meminfo_optional_metric!(
    node_memory_Shmem_bytes,
    "Amount of memory consumed in tmpfs(5) filesystems.",
    mem_shmem,
    tests_shmem
);

register_meminfo_optional_metric!(
    node_memory_ShmemHugePages_bytes,
    "Memory used by shared memory (shmem) and tmpfs(5) allocated with huge pages.",
    mem_shmem_hugepages,
    tests_shmem_hugepages
);

register_meminfo_optional_metric!(
    node_memory_ShmemPmdMapped_bytes,
    "Shared memory mapped into user space with huge pages.",
    mem_shmem_pmd_mapped,
    tests_shmem_pmd_mapped
);

register_meminfo_metric!(
    node_memory_Slab_bytes,
    "In-kernel data structures cache.",
    mem_slab,
    tests_slab
);

register_meminfo_optional_metric!(
    node_memory_SReclaimable_bytes,
    "Part of Slab that might be reclaimed.",
    mem_s_reclaimable,
    tests_s_reclaimable
);

register_meminfo_optional_metric!(
    node_memory_SUnreclaim_bytes,
    "Part of Slab that cannot be reclaimed on memory pressure.",
    mem_s_unreclaim,
    tests_s_unreclaim
);

register_meminfo_metric!(
    node_memory_SwapCached_bytes,
    "Memory that once was swapped out, is swapped back in but still also is in the swap file.",
    mem_swap_cached,
    tests_swap_cached
);

register_meminfo_metric!(
    node_memory_SwapFree_bytes,
    "Amount of swap space that is currently unused.",
    mem_swap_free,
    tests_swap_free
);

register_meminfo_metric!(
    node_memory_SwapTotal_bytes,
    "Total amount of swap space available.",
    mem_swap_total,
    tests_swap_total
);

register_meminfo_optional_metric!(
    node_memory_Unevictable_bytes,
    "Unevictable memory.",
    mem_unevictable,
    tests_unevictable
);

register_meminfo_metric!(
    node_memory_VmallocChunk_bytes,
    "Largest contiguous block of vmalloc area which is free.",
    mem_vmalloc_chunk,
    tests_vmalloc_chunk
);

register_meminfo_metric!(
    node_memory_VmallocTotal_bytes,
    "Total size of vmalloc memory area.",
    mem_vmalloc_total,
    tests_vmalloc_total
);

register_meminfo_metric!(
    node_memory_VmallocUsed_bytes,
    "Amount of vmalloc area which is used.",
    mem_vmalloc_used,
    tests_vmalloc_used
);

register_meminfo_metric!(
    node_memory_Writeback_bytes,
    "Memory which is actively being written back to the disk.",
    mem_writeback,
    tests_writeback
);

register_meminfo_optional_metric!(
    node_memory_WritebackTmp_bytes,
    "Memory used by FUSE for temporary writeback buffers.",
    mem_writeback_tmp,
    tests_writeback_tmp
);

register_meminfo_optional_metric!(
    node_memory_Zswap_bytes,
    "Memory consumed by the zswap backend (compressed size).",
    mem_z_swap,
    tests_z_swap
);

register_meminfo_optional_metric!(
    node_memory_Zswapped_bytes,
    "Amount of anonymous memory stored in zswap (original size).",
    mem_z_swapped,
    tests_z_swapped
);
