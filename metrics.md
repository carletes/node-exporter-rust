# Metrics

## To do

    # HELP node_arp_entries ARP entries by device
    # TYPE node_arp_entries gauge
    node_arp_entries{device="wlp170s0"} 1
    # HELP node_cooling_device_cur_state Current throttle state of the cooling device
    # TYPE node_cooling_device_cur_state gauge
    node_cooling_device_cur_state{name="0",type="Processor"} 0
    node_cooling_device_cur_state{name="1",type="Processor"} 0
    node_cooling_device_cur_state{name="2",type="Processor"} 0
    node_cooling_device_cur_state{name="3",type="Processor"} 0
    node_cooling_device_cur_state{name="4",type="Processor"} 0
    node_cooling_device_cur_state{name="5",type="Processor"} 0
    node_cooling_device_cur_state{name="6",type="Processor"} 0
    node_cooling_device_cur_state{name="7",type="Processor"} 0
    node_cooling_device_cur_state{name="8",type="intel_powerclamp"} -1
    node_cooling_device_cur_state{name="9",type="TCC Offset"} 0
    # HELP node_cooling_device_max_state Maximum throttle state of the cooling device
    # TYPE node_cooling_device_max_state gauge
    node_cooling_device_max_state{name="0",type="Processor"} 7
    node_cooling_device_max_state{name="1",type="Processor"} 7
    node_cooling_device_max_state{name="2",type="Processor"} 7
    node_cooling_device_max_state{name="3",type="Processor"} 7
    node_cooling_device_max_state{name="4",type="Processor"} 7
    node_cooling_device_max_state{name="5",type="Processor"} 7
    node_cooling_device_max_state{name="6",type="Processor"} 7
    node_cooling_device_max_state{name="7",type="Processor"} 7
    node_cooling_device_max_state{name="8",type="intel_powerclamp"} 50
    node_cooling_device_max_state{name="9",type="TCC Offset"} 63
    # HELP node_disk_device_mapper_info Info about disk device mapper.
    # TYPE node_disk_device_mapper_info gauge
    node_disk_device_mapper_info{device="dm-0",lv_layer="",lv_name="",name="crypted-nixos",uuid="CRYPT-LUKS1-972ae8de933f49df9e26d1465aa70cd3-crypted-nixos",vg_name=""} 1
    node_disk_device_mapper_info{device="dm-1",lv_layer="",lv_name="swap",name="vg-swap",uuid="LVM-a85BYxRMhCLeyQX2YxgeTNqRhy5Ot0T16cwpNvV6FmNnSUu3IWJcZnB0N5gJjZT5",vg_name="vg"} 1
    node_disk_device_mapper_info{device="dm-2",lv_layer="",lv_name="root",name="vg-root",uuid="LVM-a85BYxRMhCLeyQX2YxgeTNqRhy5Ot0T101XK8mfArcstm3D9fDIG2JyD0l7SUeCA",vg_name="vg"} 1
    # HELP node_disk_discard_time_seconds_total This is the total number of seconds spent by all discards.
    # TYPE node_disk_discard_time_seconds_total counter
    node_disk_discard_time_seconds_total{device="dm-0"} 0
    node_disk_discard_time_seconds_total{device="dm-1"} 0
    node_disk_discard_time_seconds_total{device="dm-2"} 0
    node_disk_discard_time_seconds_total{device="nvme0n1"} 0
    # HELP node_disk_discarded_sectors_total The total number of sectors discarded successfully.
    # TYPE node_disk_discarded_sectors_total counter
    node_disk_discarded_sectors_total{device="dm-0"} 0
    node_disk_discarded_sectors_total{device="dm-1"} 0
    node_disk_discarded_sectors_total{device="dm-2"} 0
    node_disk_discarded_sectors_total{device="nvme0n1"} 0
    # HELP node_disk_discards_completed_total The total number of discards completed successfully.
    # TYPE node_disk_discards_completed_total counter
    node_disk_discards_completed_total{device="dm-0"} 0
    node_disk_discards_completed_total{device="dm-1"} 0
    node_disk_discards_completed_total{device="dm-2"} 0
    node_disk_discards_completed_total{device="nvme0n1"} 0
    # HELP node_disk_discards_merged_total The total number of discards merged.
    # TYPE node_disk_discards_merged_total counter
    node_disk_discards_merged_total{device="dm-0"} 0
    node_disk_discards_merged_total{device="dm-1"} 0
    node_disk_discards_merged_total{device="dm-2"} 0
    node_disk_discards_merged_total{device="nvme0n1"} 0
    # HELP node_disk_filesystem_info Info about disk filesystem.
    # TYPE node_disk_filesystem_info gauge
    node_disk_filesystem_info{device="dm-0",type="LVM2_member",usage="raid",uuid="UtspfD-Cdka-1yIL-L42v-vCe6-J5zD-kGBtEs",version="LVM2 001"} 1
    node_disk_filesystem_info{device="dm-1",type="swap",usage="other",uuid="dd741fb0-ddea-46cd-ab23-e7f24df9204c",version="1"} 1
    node_disk_filesystem_info{device="dm-2",type="xfs",usage="filesystem",uuid="12410089-bc9f-442d-9019-743358bce231",version=""} 1
    # HELP node_disk_flush_requests_time_seconds_total This is the total number of seconds spent by all flush requests.
    # TYPE node_disk_flush_requests_time_seconds_total counter
    node_disk_flush_requests_time_seconds_total{device="dm-0"} 0
    node_disk_flush_requests_time_seconds_total{device="dm-1"} 0
    node_disk_flush_requests_time_seconds_total{device="dm-2"} 0
    node_disk_flush_requests_time_seconds_total{device="nvme0n1"} 14.024000000000001
    # HELP node_disk_flush_requests_total The total number of flush requests completed successfully
    # TYPE node_disk_flush_requests_total counter
    node_disk_flush_requests_total{device="dm-0"} 0
    node_disk_flush_requests_total{device="dm-1"} 0
    node_disk_flush_requests_total{device="dm-2"} 0
    node_disk_flush_requests_total{device="nvme0n1"} 4791
    # HELP node_disk_info Info of /sys/block/<block_device>.
    # TYPE node_disk_info gauge
    node_disk_info{device="dm-0",major="254",minor="0",model="",path="",revision="",serial="",wwn=""} 1
    node_disk_info{device="dm-1",major="254",minor="1",model="",path="",revision="",serial="",wwn=""} 1
    node_disk_info{device="dm-2",major="254",minor="2",model="",path="",revision="",serial="",wwn=""} 1
    node_disk_info{device="nvme0n1",major="259",minor="0",model="WD_BLACK  SN750 2TB",path="pci-0000:01:00.0-nvme-1",revision="112000WD",serial="212348800873",wwn="eui.e8238fa6bf530001001b448b4143bc95"} 1
    # HELP node_disk_io_now The number of I/Os currently in progress.
    # TYPE node_disk_io_now gauge
    node_disk_io_now{device="dm-0"} 0
    node_disk_io_now{device="dm-1"} 0
    node_disk_io_now{device="dm-2"} 0
    node_disk_io_now{device="nvme0n1"} 0
    # HELP node_disk_io_time_seconds_total Total seconds spent doing I/Os.
    # TYPE node_disk_io_time_seconds_total counter
    node_disk_io_time_seconds_total{device="dm-0"} 105.436
    node_disk_io_time_seconds_total{device="dm-1"} 0.016
    node_disk_io_time_seconds_total{device="dm-2"} 105.415
    node_disk_io_time_seconds_total{device="nvme0n1"} 103.947
    # HELP node_disk_io_time_weighted_seconds_total The weighted # of seconds spent doing I/Os.
    # TYPE node_disk_io_time_weighted_seconds_total counter
    node_disk_io_time_weighted_seconds_total{device="dm-0"} 3835.7670000000003
    node_disk_io_time_weighted_seconds_total{device="dm-1"} 0.026000000000000002
    node_disk_io_time_weighted_seconds_total{device="dm-2"} 3549.001
    node_disk_io_time_weighted_seconds_total{device="nvme0n1"} 290.02500000000003
    # HELP node_disk_read_bytes_total The total number of bytes read successfully.
    # TYPE node_disk_read_bytes_total counter
    node_disk_read_bytes_total{device="dm-0"} 2.649753088e+09
    node_disk_read_bytes_total{device="dm-1"} 2.27328e+06
    node_disk_read_bytes_total{device="dm-2"} 2.646726144e+09
    node_disk_read_bytes_total{device="nvme0n1"} 2.6577792e+09
    # HELP node_disk_read_time_seconds_total The total number of seconds spent by all reads.
    # TYPE node_disk_read_time_seconds_total counter
    node_disk_read_time_seconds_total{device="dm-0"} 25.215
    node_disk_read_time_seconds_total{device="dm-1"} 0.026000000000000002
    node_disk_read_time_seconds_total{device="dm-2"} 25.331
    node_disk_read_time_seconds_total{device="nvme0n1"} 20.094
    # HELP node_disk_reads_completed_total The total number of reads completed successfully.
    # TYPE node_disk_reads_completed_total counter
    node_disk_reads_completed_total{device="dm-0"} 84386
    node_disk_reads_completed_total{device="dm-1"} 98
    node_disk_reads_completed_total{device="dm-2"} 84228
    node_disk_reads_completed_total{device="nvme0n1"} 84605
    # HELP node_disk_reads_merged_total The total number of reads merged.
    # TYPE node_disk_reads_merged_total counter
    node_disk_reads_merged_total{device="dm-0"} 0
    node_disk_reads_merged_total{device="dm-1"} 0
    node_disk_reads_merged_total{device="dm-2"} 0
    node_disk_reads_merged_total{device="nvme0n1"} 1042
    # HELP node_disk_write_time_seconds_total This is the total number of seconds spent by all writes.
    # TYPE node_disk_write_time_seconds_total counter
    node_disk_write_time_seconds_total{device="dm-0"} 3810.552
    node_disk_write_time_seconds_total{device="dm-1"} 0
    node_disk_write_time_seconds_total{device="dm-2"} 3523.67
    node_disk_write_time_seconds_total{device="nvme0n1"} 255.906
    # HELP node_disk_writes_completed_total The total number of writes completed successfully.
    # TYPE node_disk_writes_completed_total counter
    node_disk_writes_completed_total{device="dm-0"} 263990
    node_disk_writes_completed_total{device="dm-1"} 0
    node_disk_writes_completed_total{device="dm-2"} 256583
    node_disk_writes_completed_total{device="nvme0n1"} 206936
    # HELP node_disk_writes_merged_total The number of writes merged.
    # TYPE node_disk_writes_merged_total counter
    node_disk_writes_merged_total{device="dm-0"} 0
    node_disk_writes_merged_total{device="dm-1"} 0
    node_disk_writes_merged_total{device="dm-2"} 0
    node_disk_writes_merged_total{device="nvme0n1"} 69720
    # HELP node_disk_written_bytes_total The total number of bytes written successfully.
    # TYPE node_disk_written_bytes_total counter
    node_disk_written_bytes_total{device="dm-0"} 1.5379121664e+10
    node_disk_written_bytes_total{device="dm-1"} 0
    node_disk_written_bytes_total{device="dm-2"} 1.5379121664e+10
    node_disk_written_bytes_total{device="nvme0n1"} 1.5379122688e+10
    # HELP node_dmi_info A metric with a constant '1' value labeled by bios_date, bios_release, bios_vendor, bios_version, board_asset_tag, board_name, board_serial, board_vendor, board_version, chassis_asset_tag, chassis_serial, chassis_vendor, chassis_version, product_family, product_name, product_serial, product_sku, product_uuid, product_version, system_vendor if provided by DMI.
    # TYPE node_dmi_info gauge
    node_dmi_info{bios_date="10/27/2022",bios_release="3.17",bios_vendor="INSYDE Corp.",bios_version="03.17",board_asset_tag="*",board_name="FRANBMCP0B",board_vendor="Framework",board_version="AB",chassis_asset_tag="FRANBMCPAB14440006",chassis_vendor="Framework",chassis_version="AB",product_family="FRANBMCP",product_name="Laptop",product_sku="FRANBMCP0B",product_version="AB",system_vendor="Framework"} 1
    # HELP node_entropy_available_bits Bits of available entropy.
    # TYPE node_entropy_available_bits gauge
    node_entropy_available_bits 256
    # HELP node_entropy_pool_size_bits Bits of entropy pool.
    # TYPE node_entropy_pool_size_bits gauge
    node_entropy_pool_size_bits 256
    # HELP node_exporter_build_info A metric with a constant '1' value labeled by version, revision, branch, goversion from which node_exporter was built, and the goos and goarch for the build.
    # TYPE node_exporter_build_info gauge
    node_exporter_build_info{branch="HEAD",goarch="amd64",goos="linux",goversion="go1.21.4",revision="7333465abf9efba81876303bb57e6fadb946041b",tags="netgo osusergo static_build",version="1.7.0"} 1
    # HELP node_filefd_allocated File descriptor statistics: allocated.
    # TYPE node_filefd_allocated gauge
    node_filefd_allocated 6976
    # HELP node_filefd_maximum File descriptor statistics: maximum.
    # TYPE node_filefd_maximum gauge
    node_filefd_maximum 9.223372036854776e+18
    # HELP node_filesystem_avail_bytes Filesystem space available to non-root users in bytes.
    # TYPE node_filesystem_avail_bytes gauge
    node_filesystem_avail_bytes{device="/dev/disk/by-uuid/12410089-bc9f-442d-9019-743358bce231",fstype="xfs",mountpoint="/"} 1.343623827456e+12
    node_filesystem_avail_bytes{device="/dev/disk/by-uuid/12410089-bc9f-442d-9019-743358bce231",fstype="xfs",mountpoint="/nix/store"} 1.343623827456e+12
    node_filesystem_avail_bytes{device="/dev/nvme0n1p1",fstype="vfat",mountpoint="/boot/efi"} 4.98716672e+08
    node_filesystem_avail_bytes{device="gvfsd-fuse",fstype="fuse.gvfsd-fuse",mountpoint="/run/user/1000/gvfs"} 0
    node_filesystem_avail_bytes{device="ramfs",fstype="ramfs",mountpoint="/run/keys"} 0
    node_filesystem_avail_bytes{device="tmpfs",fstype="tmpfs",mountpoint="/run"} 1.6797310976e+10
    node_filesystem_avail_bytes{device="tmpfs",fstype="tmpfs",mountpoint="/run/user/1000"} 6.721929216e+09
    node_filesystem_avail_bytes{device="tmpfs",fstype="tmpfs",mountpoint="/run/wrappers"} 3.3608450048e+10
    node_filesystem_avail_bytes{device="tmpfs",fstype="tmpfs",mountpoint="/tmp"} 3.3589850112e+10
    # HELP node_filesystem_device_error Whether an error occurred while getting statistics for the given device.
    # TYPE node_filesystem_device_error gauge
    node_filesystem_device_error{device="/dev/disk/by-uuid/12410089-bc9f-442d-9019-743358bce231",fstype="xfs",mountpoint="/"} 0
    node_filesystem_device_error{device="/dev/disk/by-uuid/12410089-bc9f-442d-9019-743358bce231",fstype="xfs",mountpoint="/nix/store"} 0
    node_filesystem_device_error{device="/dev/nvme0n1p1",fstype="vfat",mountpoint="/boot/efi"} 0
    node_filesystem_device_error{device="gvfsd-fuse",fstype="fuse.gvfsd-fuse",mountpoint="/run/user/1000/gvfs"} 0
    node_filesystem_device_error{device="portal",fstype="fuse.portal",mountpoint="/run/user/1000/doc"} 1
    node_filesystem_device_error{device="ramfs",fstype="ramfs",mountpoint="/run/keys"} 0
    node_filesystem_device_error{device="tmpfs",fstype="tmpfs",mountpoint="/run"} 0
    node_filesystem_device_error{device="tmpfs",fstype="tmpfs",mountpoint="/run/user/1000"} 0
    node_filesystem_device_error{device="tmpfs",fstype="tmpfs",mountpoint="/run/wrappers"} 0
    node_filesystem_device_error{device="tmpfs",fstype="tmpfs",mountpoint="/tmp"} 0
    # HELP node_filesystem_files Filesystem total file nodes.
    # TYPE node_filesystem_files gauge
    node_filesystem_files{device="/dev/disk/by-uuid/12410089-bc9f-442d-9019-743358bce231",fstype="xfs",mountpoint="/"} 1.9487904e+08
    node_filesystem_files{device="/dev/disk/by-uuid/12410089-bc9f-442d-9019-743358bce231",fstype="xfs",mountpoint="/nix/store"} 1.9487904e+08
    node_filesystem_files{device="/dev/nvme0n1p1",fstype="vfat",mountpoint="/boot/efi"} 0
    node_filesystem_files{device="gvfsd-fuse",fstype="fuse.gvfsd-fuse",mountpoint="/run/user/1000/gvfs"} 0
    node_filesystem_files{device="ramfs",fstype="ramfs",mountpoint="/run/keys"} 0
    node_filesystem_files{device="tmpfs",fstype="tmpfs",mountpoint="/run"} 8.205524e+06
    node_filesystem_files{device="tmpfs",fstype="tmpfs",mountpoint="/run/user/1000"} 1.641104e+06
    node_filesystem_files{device="tmpfs",fstype="tmpfs",mountpoint="/run/wrappers"} 8.205524e+06
    node_filesystem_files{device="tmpfs",fstype="tmpfs",mountpoint="/tmp"} 8.205524e+06
    # HELP node_filesystem_files_free Filesystem total free file nodes.
    # TYPE node_filesystem_files_free gauge
    node_filesystem_files_free{device="/dev/disk/by-uuid/12410089-bc9f-442d-9019-743358bce231",fstype="xfs",mountpoint="/"} 1.88915045e+08
    node_filesystem_files_free{device="/dev/disk/by-uuid/12410089-bc9f-442d-9019-743358bce231",fstype="xfs",mountpoint="/nix/store"} 1.88915045e+08
    node_filesystem_files_free{device="/dev/nvme0n1p1",fstype="vfat",mountpoint="/boot/efi"} 0
    node_filesystem_files_free{device="gvfsd-fuse",fstype="fuse.gvfsd-fuse",mountpoint="/run/user/1000/gvfs"} 0
    node_filesystem_files_free{device="ramfs",fstype="ramfs",mountpoint="/run/keys"} 0
    node_filesystem_files_free{device="tmpfs",fstype="tmpfs",mountpoint="/run"} 8.203404e+06
    node_filesystem_files_free{device="tmpfs",fstype="tmpfs",mountpoint="/run/user/1000"} 1.641039e+06
    node_filesystem_files_free{device="tmpfs",fstype="tmpfs",mountpoint="/run/wrappers"} 8.2055e+06
    node_filesystem_files_free{device="tmpfs",fstype="tmpfs",mountpoint="/tmp"} 8.205457e+06
    # HELP node_filesystem_free_bytes Filesystem free space in bytes.
    # TYPE node_filesystem_free_bytes gauge
    node_filesystem_free_bytes{device="/dev/disk/by-uuid/12410089-bc9f-442d-9019-743358bce231",fstype="xfs",mountpoint="/"} 1.343623827456e+12
    node_filesystem_free_bytes{device="/dev/disk/by-uuid/12410089-bc9f-442d-9019-743358bce231",fstype="xfs",mountpoint="/nix/store"} 1.343623827456e+12
    node_filesystem_free_bytes{device="/dev/nvme0n1p1",fstype="vfat",mountpoint="/boot/efi"} 4.98716672e+08
    node_filesystem_free_bytes{device="gvfsd-fuse",fstype="fuse.gvfsd-fuse",mountpoint="/run/user/1000/gvfs"} 0
    node_filesystem_free_bytes{device="ramfs",fstype="ramfs",mountpoint="/run/keys"} 0
    node_filesystem_free_bytes{device="tmpfs",fstype="tmpfs",mountpoint="/run"} 1.6797310976e+10
    node_filesystem_free_bytes{device="tmpfs",fstype="tmpfs",mountpoint="/run/user/1000"} 6.721929216e+09
    node_filesystem_free_bytes{device="tmpfs",fstype="tmpfs",mountpoint="/run/wrappers"} 3.3608450048e+10
    node_filesystem_free_bytes{device="tmpfs",fstype="tmpfs",mountpoint="/tmp"} 3.3589850112e+10
    # HELP node_filesystem_readonly Filesystem read-only status.
    # TYPE node_filesystem_readonly gauge
    node_filesystem_readonly{device="/dev/disk/by-uuid/12410089-bc9f-442d-9019-743358bce231",fstype="xfs",mountpoint="/"} 0
    node_filesystem_readonly{device="/dev/disk/by-uuid/12410089-bc9f-442d-9019-743358bce231",fstype="xfs",mountpoint="/nix/store"} 1
    node_filesystem_readonly{device="/dev/nvme0n1p1",fstype="vfat",mountpoint="/boot/efi"} 0
    node_filesystem_readonly{device="gvfsd-fuse",fstype="fuse.gvfsd-fuse",mountpoint="/run/user/1000/gvfs"} 0
    node_filesystem_readonly{device="portal",fstype="fuse.portal",mountpoint="/run/user/1000/doc"} 0
    node_filesystem_readonly{device="ramfs",fstype="ramfs",mountpoint="/run/keys"} 0
    node_filesystem_readonly{device="tmpfs",fstype="tmpfs",mountpoint="/run"} 0
    node_filesystem_readonly{device="tmpfs",fstype="tmpfs",mountpoint="/run/user/1000"} 0
    node_filesystem_readonly{device="tmpfs",fstype="tmpfs",mountpoint="/run/wrappers"} 0
    node_filesystem_readonly{device="tmpfs",fstype="tmpfs",mountpoint="/tmp"} 0
    # HELP node_filesystem_size_bytes Filesystem size in bytes.
    # TYPE node_filesystem_size_bytes gauge
    node_filesystem_size_bytes{device="/dev/disk/by-uuid/12410089-bc9f-442d-9019-743358bce231",fstype="xfs",mountpoint="/"} 1.994587369472e+12
    node_filesystem_size_bytes{device="/dev/disk/by-uuid/12410089-bc9f-442d-9019-743358bce231",fstype="xfs",mountpoint="/nix/store"} 1.994587369472e+12
    node_filesystem_size_bytes{device="/dev/nvme0n1p1",fstype="vfat",mountpoint="/boot/efi"} 5.34757376e+08
    node_filesystem_size_bytes{device="gvfsd-fuse",fstype="fuse.gvfsd-fuse",mountpoint="/run/user/1000/gvfs"} 0
    node_filesystem_size_bytes{device="ramfs",fstype="ramfs",mountpoint="/run/keys"} 0
    node_filesystem_size_bytes{device="tmpfs",fstype="tmpfs",mountpoint="/run"} 1.6804913152e+10
    node_filesystem_size_bytes{device="tmpfs",fstype="tmpfs",mountpoint="/run/user/1000"} 6.721961984e+09
    node_filesystem_size_bytes{device="tmpfs",fstype="tmpfs",mountpoint="/run/wrappers"} 3.3609826304e+10
    node_filesystem_size_bytes{device="tmpfs",fstype="tmpfs",mountpoint="/tmp"} 3.3609826304e+10
    # HELP node_forks_total Total number of forks.
    # TYPE node_forks_total counter
    node_forks_total 193529
    # HELP node_hwmon_chip_names Annotation metric for human-readable chip names
    # TYPE node_hwmon_chip_names gauge
    node_hwmon_chip_names{chip="nvme_nvme0",chip_name="nvme"} 1
    node_hwmon_chip_names{chip="platform_coretemp_0",chip_name="coretemp"} 1
    node_hwmon_chip_names{chip="power_supply_acad",chip_name="acad"} 1
    node_hwmon_chip_names{chip="power_supply_bat1",chip_name="bat1"} 1
    node_hwmon_chip_names{chip="thermal_thermal_zone2",chip_name="acpitz"} 1
    node_hwmon_chip_names{chip="thermal_thermal_zone6",chip_name="iwlwifi_1"} 1
    # HELP node_hwmon_curr_amps Hardware monitor for current (input)
    # TYPE node_hwmon_curr_amps gauge
    node_hwmon_curr_amps{chip="power_supply_bat1",sensor="curr1"} 2.814
    # HELP node_hwmon_in_volts Hardware monitor for voltage (input)
    # TYPE node_hwmon_in_volts gauge
    node_hwmon_in_volts{chip="power_supply_bat1",sensor="in0"} 16.599
    # HELP node_hwmon_sensor_label Label for given chip and sensor
    # TYPE node_hwmon_sensor_label gauge
    node_hwmon_sensor_label{chip="nvme_nvme0",label="Composite",sensor="temp1"} 1
    node_hwmon_sensor_label{chip="platform_coretemp_0",label="Core 0",sensor="temp2"} 1
    node_hwmon_sensor_label{chip="platform_coretemp_0",label="Core 1",sensor="temp3"} 1
    node_hwmon_sensor_label{chip="platform_coretemp_0",label="Core 2",sensor="temp4"} 1
    node_hwmon_sensor_label{chip="platform_coretemp_0",label="Core 3",sensor="temp5"} 1
    node_hwmon_sensor_label{chip="platform_coretemp_0",label="Package id 0",sensor="temp1"} 1
    # HELP node_hwmon_temp_alarm Hardware sensor alarm status (temp)
    # TYPE node_hwmon_temp_alarm gauge
    node_hwmon_temp_alarm{chip="nvme_nvme0",sensor="temp1"} 0
    # HELP node_hwmon_temp_celsius Hardware monitor for temperature (input)
    # TYPE node_hwmon_temp_celsius gauge
    node_hwmon_temp_celsius{chip="nvme_nvme0",sensor="temp1"} 43.85
    node_hwmon_temp_celsius{chip="platform_coretemp_0",sensor="temp1"} 41
    node_hwmon_temp_celsius{chip="platform_coretemp_0",sensor="temp2"} 38
    node_hwmon_temp_celsius{chip="platform_coretemp_0",sensor="temp3"} 38
    node_hwmon_temp_celsius{chip="platform_coretemp_0",sensor="temp4"} 39
    node_hwmon_temp_celsius{chip="platform_coretemp_0",sensor="temp5"} 36
    node_hwmon_temp_celsius{chip="thermal_thermal_zone2",sensor="temp0"} 41.800000000000004
    node_hwmon_temp_celsius{chip="thermal_thermal_zone2",sensor="temp1"} 41.800000000000004
    node_hwmon_temp_celsius{chip="thermal_thermal_zone6",sensor="temp0"} 44
    node_hwmon_temp_celsius{chip="thermal_thermal_zone6",sensor="temp1"} 44
    # HELP node_hwmon_temp_crit_alarm_celsius Hardware monitor for temperature (crit_alarm)
    # TYPE node_hwmon_temp_crit_alarm_celsius gauge
    node_hwmon_temp_crit_alarm_celsius{chip="platform_coretemp_0",sensor="temp1"} 0
    node_hwmon_temp_crit_alarm_celsius{chip="platform_coretemp_0",sensor="temp2"} 0
    node_hwmon_temp_crit_alarm_celsius{chip="platform_coretemp_0",sensor="temp3"} 0
    node_hwmon_temp_crit_alarm_celsius{chip="platform_coretemp_0",sensor="temp4"} 0
    node_hwmon_temp_crit_alarm_celsius{chip="platform_coretemp_0",sensor="temp5"} 0
    # HELP node_hwmon_temp_crit_celsius Hardware monitor for temperature (crit)
    # TYPE node_hwmon_temp_crit_celsius gauge
    node_hwmon_temp_crit_celsius{chip="nvme_nvme0",sensor="temp1"} 87.85000000000001
    node_hwmon_temp_crit_celsius{chip="platform_coretemp_0",sensor="temp1"} 100
    node_hwmon_temp_crit_celsius{chip="platform_coretemp_0",sensor="temp2"} 100
    node_hwmon_temp_crit_celsius{chip="platform_coretemp_0",sensor="temp3"} 100
    node_hwmon_temp_crit_celsius{chip="platform_coretemp_0",sensor="temp4"} 100
    node_hwmon_temp_crit_celsius{chip="platform_coretemp_0",sensor="temp5"} 100
    node_hwmon_temp_crit_celsius{chip="thermal_thermal_zone2",sensor="temp1"} 210
    # HELP node_hwmon_temp_max_celsius Hardware monitor for temperature (max)
    # TYPE node_hwmon_temp_max_celsius gauge
    node_hwmon_temp_max_celsius{chip="nvme_nvme0",sensor="temp1"} 83.85000000000001
    node_hwmon_temp_max_celsius{chip="platform_coretemp_0",sensor="temp1"} 100
    node_hwmon_temp_max_celsius{chip="platform_coretemp_0",sensor="temp2"} 100
    node_hwmon_temp_max_celsius{chip="platform_coretemp_0",sensor="temp3"} 100
    node_hwmon_temp_max_celsius{chip="platform_coretemp_0",sensor="temp4"} 100
    node_hwmon_temp_max_celsius{chip="platform_coretemp_0",sensor="temp5"} 100
    # HELP node_hwmon_temp_min_celsius Hardware monitor for temperature (min)
    # TYPE node_hwmon_temp_min_celsius gauge
    node_hwmon_temp_min_celsius{chip="nvme_nvme0",sensor="temp1"} -5.15
    # HELP node_intr_total Total number of interrupts serviced.
    # TYPE node_intr_total counter
    node_intr_total 2.3324872e+07
    # HELP node_memory_Hugepagesize_bytes Memory information field Hugepagesize_bytes.
    # TYPE node_memory_Hugepagesize_bytes gauge
    node_memory_Hugepagesize_bytes 2.097152e+06
    # HELP node_memory_Hugetlb_bytes Memory information field Hugetlb_bytes.
    # TYPE node_memory_Hugetlb_bytes gauge
    node_memory_Hugetlb_bytes 0
    # HELP node_memory_Inactive_anon_bytes Memory information field Inactive_anon_bytes.
    # TYPE node_memory_Inactive_anon_bytes gauge
    node_memory_Inactive_anon_bytes 0
    # HELP node_memory_Inactive_bytes Memory information field Inactive_bytes.
    # TYPE node_memory_Inactive_bytes gauge
    node_memory_Inactive_bytes 9.321775104e+09
    # HELP node_memory_Inactive_file_bytes Memory information field Inactive_file_bytes.
    # TYPE node_memory_Inactive_file_bytes gauge
    node_memory_Inactive_file_bytes 9.321775104e+09
    # HELP node_memory_KReclaimable_bytes Memory information field KReclaimable_bytes.
    # TYPE node_memory_KReclaimable_bytes gauge
    node_memory_KReclaimable_bytes 4.23129088e+08
    # HELP node_memory_KernelStack_bytes Memory information field KernelStack_bytes.
    # TYPE node_memory_KernelStack_bytes gauge
    node_memory_KernelStack_bytes 1.6269312e+07
    # HELP node_memory_Mapped_bytes Memory information field Mapped_bytes.
    # TYPE node_memory_Mapped_bytes gauge
    node_memory_Mapped_bytes 5.48458496e+08
    # HELP node_memory_MemAvailable_bytes Memory information field MemAvailable_bytes.
    # TYPE node_memory_MemAvailable_bytes gauge
    node_memory_MemAvailable_bytes 6.1462888448e+10
    # HELP node_memory_MemFree_bytes Memory information field MemFree_bytes.
    # TYPE node_memory_MemFree_bytes gauge
    node_memory_MemFree_bytes 5.0643914752e+10
    # HELP node_memory_MemTotal_bytes Memory information field MemTotal_bytes.
    # TYPE node_memory_MemTotal_bytes gauge
    node_memory_MemTotal_bytes 6.7219652608e+10
    # HELP node_memory_Mlocked_bytes Memory information field Mlocked_bytes.
    # TYPE node_memory_Mlocked_bytes gauge
    node_memory_Mlocked_bytes 0
    # HELP node_memory_NFS_Unstable_bytes Memory information field NFS_Unstable_bytes.
    # TYPE node_memory_NFS_Unstable_bytes gauge
    node_memory_NFS_Unstable_bytes 0
    # HELP node_memory_PageTables_bytes Memory information field PageTables_bytes.
    # TYPE node_memory_PageTables_bytes gauge
    node_memory_PageTables_bytes 3.21536e+07
    # HELP node_memory_Percpu_bytes Memory information field Percpu_bytes.
    # TYPE node_memory_Percpu_bytes gauge
    node_memory_Percpu_bytes 5.40672e+06
    # HELP node_memory_SReclaimable_bytes Memory information field SReclaimable_bytes.
    # TYPE node_memory_SReclaimable_bytes gauge
    node_memory_SReclaimable_bytes 4.23129088e+08
    # HELP node_memory_SUnreclaim_bytes Memory information field SUnreclaim_bytes.
    # TYPE node_memory_SUnreclaim_bytes gauge
    node_memory_SUnreclaim_bytes 3.22433024e+08
    # HELP node_memory_SecPageTables_bytes Memory information field SecPageTables_bytes.
    # TYPE node_memory_SecPageTables_bytes gauge
    node_memory_SecPageTables_bytes 0
    # HELP node_memory_ShmemHugePages_bytes Memory information field ShmemHugePages_bytes.
    # TYPE node_memory_ShmemHugePages_bytes gauge
    node_memory_ShmemHugePages_bytes 3.16669952e+08
    # HELP node_memory_ShmemPmdMapped_bytes Memory information field ShmemPmdMapped_bytes.
    # TYPE node_memory_ShmemPmdMapped_bytes gauge
    node_memory_ShmemPmdMapped_bytes 0
    # HELP node_memory_Shmem_bytes Memory information field Shmem_bytes.
    # TYPE node_memory_Shmem_bytes gauge
    node_memory_Shmem_bytes 4.37460992e+08
    # HELP node_memory_Slab_bytes Memory information field Slab_bytes.
    # TYPE node_memory_Slab_bytes gauge
    node_memory_Slab_bytes 7.45562112e+08
    # HELP node_memory_SwapCached_bytes Memory information field SwapCached_bytes.
    # TYPE node_memory_SwapCached_bytes gauge
    node_memory_SwapCached_bytes 0
    # HELP node_memory_SwapFree_bytes Memory information field SwapFree_bytes.
    # TYPE node_memory_SwapFree_bytes gauge
    node_memory_SwapFree_bytes 4.2949632e+09
    # HELP node_memory_SwapTotal_bytes Memory information field SwapTotal_bytes.
    # TYPE node_memory_SwapTotal_bytes gauge
    node_memory_SwapTotal_bytes 4.2949632e+09
    # HELP node_memory_Unevictable_bytes Memory information field Unevictable_bytes.
    # TYPE node_memory_Unevictable_bytes gauge
    node_memory_Unevictable_bytes 3.59837696e+08
    # HELP node_memory_VmallocChunk_bytes Memory information field VmallocChunk_bytes.
    # TYPE node_memory_VmallocChunk_bytes gauge
    node_memory_VmallocChunk_bytes 0
    # HELP node_memory_VmallocTotal_bytes Memory information field VmallocTotal_bytes.
    # TYPE node_memory_VmallocTotal_bytes gauge
    node_memory_VmallocTotal_bytes 3.5184372087808e+13
    # HELP node_memory_VmallocUsed_bytes Memory information field VmallocUsed_bytes.
    # TYPE node_memory_VmallocUsed_bytes gauge
    node_memory_VmallocUsed_bytes 5.0839552e+07
    # HELP node_memory_WritebackTmp_bytes Memory information field WritebackTmp_bytes.
    # TYPE node_memory_WritebackTmp_bytes gauge
    node_memory_WritebackTmp_bytes 0
    # HELP node_memory_Writeback_bytes Memory information field Writeback_bytes.
    # TYPE node_memory_Writeback_bytes gauge
    node_memory_Writeback_bytes 0
    # HELP node_memory_Zswap_bytes Memory information field Zswap_bytes.
    # TYPE node_memory_Zswap_bytes gauge
    node_memory_Zswap_bytes 0
    # HELP node_memory_Zswapped_bytes Memory information field Zswapped_bytes.
    # TYPE node_memory_Zswapped_bytes gauge
    node_memory_Zswapped_bytes 0
    # HELP node_netstat_Icmp6_InErrors Statistic Icmp6InErrors.
    # TYPE node_netstat_Icmp6_InErrors untyped
    node_netstat_Icmp6_InErrors 0
    # HELP node_netstat_Icmp6_InMsgs Statistic Icmp6InMsgs.
    # TYPE node_netstat_Icmp6_InMsgs untyped
    node_netstat_Icmp6_InMsgs 5077
    # HELP node_netstat_Icmp6_OutMsgs Statistic Icmp6OutMsgs.
    # TYPE node_netstat_Icmp6_OutMsgs untyped
    node_netstat_Icmp6_OutMsgs 4576
    # HELP node_netstat_Icmp_InErrors Statistic IcmpInErrors.
    # TYPE node_netstat_Icmp_InErrors untyped
    node_netstat_Icmp_InErrors 0
    # HELP node_netstat_Icmp_InMsgs Statistic IcmpInMsgs.
    # TYPE node_netstat_Icmp_InMsgs untyped
    node_netstat_Icmp_InMsgs 59
    # HELP node_netstat_Icmp_OutMsgs Statistic IcmpOutMsgs.
    # TYPE node_netstat_Icmp_OutMsgs untyped
    node_netstat_Icmp_OutMsgs 59
    # HELP node_netstat_Ip6_InOctets Statistic Ip6InOctets.
    # TYPE node_netstat_Ip6_InOctets untyped
    node_netstat_Ip6_InOctets 3.15977313e+08
    # HELP node_netstat_Ip6_OutOctets Statistic Ip6OutOctets.
    # TYPE node_netstat_Ip6_OutOctets untyped
    node_netstat_Ip6_OutOctets 3.579016e+06
    # HELP node_netstat_IpExt_InOctets Statistic IpExtInOctets.
    # TYPE node_netstat_IpExt_InOctets untyped
    node_netstat_IpExt_InOctets 1.031820919e+09
    # HELP node_netstat_IpExt_OutOctets Statistic IpExtOutOctets.
    # TYPE node_netstat_IpExt_OutOctets untyped
    node_netstat_IpExt_OutOctets 2.1269048e+07
    # HELP node_netstat_Ip_Forwarding Statistic IpForwarding.
    # TYPE node_netstat_Ip_Forwarding untyped
    node_netstat_Ip_Forwarding 1
    # HELP node_netstat_TcpExt_ListenDrops Statistic TcpExtListenDrops.
    # TYPE node_netstat_TcpExt_ListenDrops untyped
    node_netstat_TcpExt_ListenDrops 0
    # HELP node_netstat_TcpExt_ListenOverflows Statistic TcpExtListenOverflows.
    # TYPE node_netstat_TcpExt_ListenOverflows untyped
    node_netstat_TcpExt_ListenOverflows 0
    # HELP node_netstat_TcpExt_SyncookiesFailed Statistic TcpExtSyncookiesFailed.
    # TYPE node_netstat_TcpExt_SyncookiesFailed untyped
    node_netstat_TcpExt_SyncookiesFailed 0
    # HELP node_netstat_TcpExt_SyncookiesRecv Statistic TcpExtSyncookiesRecv.
    # TYPE node_netstat_TcpExt_SyncookiesRecv untyped
    node_netstat_TcpExt_SyncookiesRecv 0
    # HELP node_netstat_TcpExt_SyncookiesSent Statistic TcpExtSyncookiesSent.
    # TYPE node_netstat_TcpExt_SyncookiesSent untyped
    node_netstat_TcpExt_SyncookiesSent 0
    # HELP node_netstat_TcpExt_TCPSynRetrans Statistic TcpExtTCPSynRetrans.
    # TYPE node_netstat_TcpExt_TCPSynRetrans untyped
    node_netstat_TcpExt_TCPSynRetrans 22
    # HELP node_netstat_TcpExt_TCPTimeouts Statistic TcpExtTCPTimeouts.
    # TYPE node_netstat_TcpExt_TCPTimeouts untyped
    node_netstat_TcpExt_TCPTimeouts 44
    # HELP node_netstat_Tcp_ActiveOpens Statistic TcpActiveOpens.
    # TYPE node_netstat_Tcp_ActiveOpens untyped
    node_netstat_Tcp_ActiveOpens 830
    # HELP node_netstat_Tcp_CurrEstab Statistic TcpCurrEstab.
    # TYPE node_netstat_Tcp_CurrEstab untyped
    node_netstat_Tcp_CurrEstab 4
    # HELP node_netstat_Tcp_InErrs Statistic TcpInErrs.
    # TYPE node_netstat_Tcp_InErrs untyped
    node_netstat_Tcp_InErrs 2
    # HELP node_netstat_Tcp_InSegs Statistic TcpInSegs.
    # TYPE node_netstat_Tcp_InSegs untyped
    node_netstat_Tcp_InSegs 458149
    # HELP node_netstat_Tcp_OutRsts Statistic TcpOutRsts.
    # TYPE node_netstat_Tcp_OutRsts untyped
    node_netstat_Tcp_OutRsts 1537
    # HELP node_netstat_Tcp_OutSegs Statistic TcpOutSegs.
    # TYPE node_netstat_Tcp_OutSegs untyped
    node_netstat_Tcp_OutSegs 358525
    # HELP node_netstat_Tcp_PassiveOpens Statistic TcpPassiveOpens.
    # TYPE node_netstat_Tcp_PassiveOpens untyped
    node_netstat_Tcp_PassiveOpens 51
    # HELP node_netstat_Tcp_RetransSegs Statistic TcpRetransSegs.
    # TYPE node_netstat_Tcp_RetransSegs untyped
    node_netstat_Tcp_RetransSegs 201
    # HELP node_netstat_Udp6_InDatagrams Statistic Udp6InDatagrams.
    # TYPE node_netstat_Udp6_InDatagrams untyped
    node_netstat_Udp6_InDatagrams 7064
    # HELP node_netstat_Udp6_InErrors Statistic Udp6InErrors.
    # TYPE node_netstat_Udp6_InErrors untyped
    node_netstat_Udp6_InErrors 0
    # HELP node_netstat_Udp6_NoPorts Statistic Udp6NoPorts.
    # TYPE node_netstat_Udp6_NoPorts untyped
    node_netstat_Udp6_NoPorts 3
    # HELP node_netstat_Udp6_OutDatagrams Statistic Udp6OutDatagrams.
    # TYPE node_netstat_Udp6_OutDatagrams untyped
    node_netstat_Udp6_OutDatagrams 2284
    # HELP node_netstat_Udp6_RcvbufErrors Statistic Udp6RcvbufErrors.
    # TYPE node_netstat_Udp6_RcvbufErrors untyped
    node_netstat_Udp6_RcvbufErrors 0
    # HELP node_netstat_Udp6_SndbufErrors Statistic Udp6SndbufErrors.
    # TYPE node_netstat_Udp6_SndbufErrors untyped
    node_netstat_Udp6_SndbufErrors 0
    # HELP node_netstat_UdpLite6_InErrors Statistic UdpLite6InErrors.
    # TYPE node_netstat_UdpLite6_InErrors untyped
    node_netstat_UdpLite6_InErrors 0
    # HELP node_netstat_UdpLite_InErrors Statistic UdpLiteInErrors.
    # TYPE node_netstat_UdpLite_InErrors untyped
    node_netstat_UdpLite_InErrors 0
    # HELP node_netstat_Udp_InDatagrams Statistic UdpInDatagrams.
    # TYPE node_netstat_Udp_InDatagrams untyped
    node_netstat_Udp_InDatagrams 7532
    # HELP node_netstat_Udp_InErrors Statistic UdpInErrors.
    # TYPE node_netstat_Udp_InErrors untyped
    node_netstat_Udp_InErrors 0
    # HELP node_netstat_Udp_NoPorts Statistic UdpNoPorts.
    # TYPE node_netstat_Udp_NoPorts untyped
    node_netstat_Udp_NoPorts 40
    # HELP node_netstat_Udp_OutDatagrams Statistic UdpOutDatagrams.
    # TYPE node_netstat_Udp_OutDatagrams untyped
    node_netstat_Udp_OutDatagrams 3674
    # HELP node_netstat_Udp_RcvbufErrors Statistic UdpRcvbufErrors.
    # TYPE node_netstat_Udp_RcvbufErrors untyped
    node_netstat_Udp_RcvbufErrors 0
    # HELP node_netstat_Udp_SndbufErrors Statistic UdpSndbufErrors.
    # TYPE node_netstat_Udp_SndbufErrors untyped
    node_netstat_Udp_SndbufErrors 0
    # HELP node_network_address_assign_type Network device property: address_assign_type
    # TYPE node_network_address_assign_type gauge
    node_network_address_assign_type{device="br-c4f03cf75ed7"} 3
    node_network_address_assign_type{device="docker0"} 3
    node_network_address_assign_type{device="lo"} 0
    node_network_address_assign_type{device="wlp170s0"} 3
    # HELP node_network_carrier Network device property: carrier
    # TYPE node_network_carrier gauge
    node_network_carrier{device="br-c4f03cf75ed7"} 0
    node_network_carrier{device="docker0"} 0
    node_network_carrier{device="lo"} 1
    node_network_carrier{device="wlp170s0"} 1
    # HELP node_network_carrier_changes_total Network device property: carrier_changes_total
    # TYPE node_network_carrier_changes_total counter
    node_network_carrier_changes_total{device="br-c4f03cf75ed7"} 1
    node_network_carrier_changes_total{device="docker0"} 11
    node_network_carrier_changes_total{device="lo"} 0
    node_network_carrier_changes_total{device="wlp170s0"} 2
    # HELP node_network_carrier_down_changes_total Network device property: carrier_down_changes_total
    # TYPE node_network_carrier_down_changes_total counter
    node_network_carrier_down_changes_total{device="br-c4f03cf75ed7"} 1
    node_network_carrier_down_changes_total{device="docker0"} 6
    node_network_carrier_down_changes_total{device="lo"} 0
    node_network_carrier_down_changes_total{device="wlp170s0"} 1
    # HELP node_network_carrier_up_changes_total Network device property: carrier_up_changes_total
    # TYPE node_network_carrier_up_changes_total counter
    node_network_carrier_up_changes_total{device="br-c4f03cf75ed7"} 0
    node_network_carrier_up_changes_total{device="docker0"} 5
    node_network_carrier_up_changes_total{device="lo"} 0
    node_network_carrier_up_changes_total{device="wlp170s0"} 1
    # HELP node_network_device_id Network device property: device_id
    # TYPE node_network_device_id gauge
    node_network_device_id{device="br-c4f03cf75ed7"} 0
    node_network_device_id{device="docker0"} 0
    node_network_device_id{device="lo"} 0
    node_network_device_id{device="wlp170s0"} 0
    # HELP node_network_dormant Network device property: dormant
    # TYPE node_network_dormant gauge
    node_network_dormant{device="br-c4f03cf75ed7"} 0
    node_network_dormant{device="docker0"} 0
    node_network_dormant{device="lo"} 0
    node_network_dormant{device="wlp170s0"} 0
    # HELP node_network_flags Network device property: flags
    # TYPE node_network_flags gauge
    node_network_flags{device="br-c4f03cf75ed7"} 4099
    node_network_flags{device="docker0"} 4099
    node_network_flags{device="lo"} 9
    node_network_flags{device="wlp170s0"} 4099
    # HELP node_network_iface_id Network device property: iface_id
    # TYPE node_network_iface_id gauge
    node_network_iface_id{device="br-c4f03cf75ed7"} 4
    node_network_iface_id{device="docker0"} 3
    node_network_iface_id{device="lo"} 1
    node_network_iface_id{device="wlp170s0"} 2
    # HELP node_network_iface_link Network device property: iface_link
    # TYPE node_network_iface_link gauge
    node_network_iface_link{device="br-c4f03cf75ed7"} 4
    node_network_iface_link{device="docker0"} 3
    node_network_iface_link{device="lo"} 1
    node_network_iface_link{device="wlp170s0"} 2
    # HELP node_network_iface_link_mode Network device property: iface_link_mode
    # TYPE node_network_iface_link_mode gauge
    node_network_iface_link_mode{device="br-c4f03cf75ed7"} 0
    node_network_iface_link_mode{device="docker0"} 0
    node_network_iface_link_mode{device="lo"} 0
    node_network_iface_link_mode{device="wlp170s0"} 1
    # HELP node_network_info Non-numeric data from /sys/class/net/<iface>, value is always 1.
    # TYPE node_network_info gauge
    node_network_info{address="00:00:00:00:00:00",adminstate="up",broadcast="00:00:00:00:00:00",device="lo",duplex="",ifalias="",operstate="unknown"} 1
    node_network_info{address="02:42:45:ca:e9:15",adminstate="up",broadcast="ff:ff:ff:ff:ff:ff",device="br-c4f03cf75ed7",duplex="unknown",ifalias="",operstate="down"} 1
    node_network_info{address="02:42:9a:7f:85:41",adminstate="up",broadcast="ff:ff:ff:ff:ff:ff",device="docker0",duplex="unknown",ifalias="",operstate="down"} 1
    node_network_info{address="4c:77:cb:1d:89:a4",adminstate="up",broadcast="ff:ff:ff:ff:ff:ff",device="wlp170s0",duplex="",ifalias="",operstate="up"} 1
    # HELP node_network_mtu_bytes Network device property: mtu_bytes
    # TYPE node_network_mtu_bytes gauge
    node_network_mtu_bytes{device="br-c4f03cf75ed7"} 1500
    node_network_mtu_bytes{device="docker0"} 1500
    node_network_mtu_bytes{device="lo"} 65536
    node_network_mtu_bytes{device="wlp170s0"} 1500
    # HELP node_network_name_assign_type Network device property: name_assign_type
    # TYPE node_network_name_assign_type gauge
    node_network_name_assign_type{device="br-c4f03cf75ed7"} 3
    node_network_name_assign_type{device="docker0"} 3
    node_network_name_assign_type{device="lo"} 2
    node_network_name_assign_type{device="wlp170s0"} 4
    # HELP node_network_net_dev_group Network device property: net_dev_group
    # TYPE node_network_net_dev_group gauge
    node_network_net_dev_group{device="br-c4f03cf75ed7"} 0
    node_network_net_dev_group{device="docker0"} 0
    node_network_net_dev_group{device="lo"} 0
    node_network_net_dev_group{device="wlp170s0"} 0
    # HELP node_network_protocol_type Network device property: protocol_type
    # TYPE node_network_protocol_type gauge
    node_network_protocol_type{device="br-c4f03cf75ed7"} 1
    node_network_protocol_type{device="docker0"} 1
    node_network_protocol_type{device="lo"} 772
    node_network_protocol_type{device="wlp170s0"} 1
    # HELP node_network_receive_bytes_total Network device statistic receive_bytes.
    # TYPE node_network_receive_bytes_total counter
    node_network_receive_bytes_total{device="br-c4f03cf75ed7"} 0
    node_network_receive_bytes_total{device="docker0"} 0
    node_network_receive_bytes_total{device="lo"} 751268
    node_network_receive_bytes_total{device="wlp170s0"} 1.411106229e+09
    # HELP node_network_receive_compressed_total Network device statistic receive_compressed.
    # TYPE node_network_receive_compressed_total counter
    node_network_receive_compressed_total{device="br-c4f03cf75ed7"} 0
    node_network_receive_compressed_total{device="docker0"} 0
    node_network_receive_compressed_total{device="lo"} 0
    node_network_receive_compressed_total{device="wlp170s0"} 0
    # HELP node_network_receive_drop_total Network device statistic receive_drop.
    # TYPE node_network_receive_drop_total counter
    node_network_receive_drop_total{device="br-c4f03cf75ed7"} 0
    node_network_receive_drop_total{device="docker0"} 0
    node_network_receive_drop_total{device="lo"} 0
    node_network_receive_drop_total{device="wlp170s0"} 0
    # HELP node_network_receive_errs_total Network device statistic receive_errs.
    # TYPE node_network_receive_errs_total counter
    node_network_receive_errs_total{device="br-c4f03cf75ed7"} 0
    node_network_receive_errs_total{device="docker0"} 0
    node_network_receive_errs_total{device="lo"} 0
    node_network_receive_errs_total{device="wlp170s0"} 0
    # HELP node_network_receive_fifo_total Network device statistic receive_fifo.
    # TYPE node_network_receive_fifo_total counter
    node_network_receive_fifo_total{device="br-c4f03cf75ed7"} 0
    node_network_receive_fifo_total{device="docker0"} 0
    node_network_receive_fifo_total{device="lo"} 0
    node_network_receive_fifo_total{device="wlp170s0"} 0
    # HELP node_network_receive_frame_total Network device statistic receive_frame.
    # TYPE node_network_receive_frame_total counter
    node_network_receive_frame_total{device="br-c4f03cf75ed7"} 0
    node_network_receive_frame_total{device="docker0"} 0
    node_network_receive_frame_total{device="lo"} 0
    node_network_receive_frame_total{device="wlp170s0"} 0
    # HELP node_network_receive_multicast_total Network device statistic receive_multicast.
    # TYPE node_network_receive_multicast_total counter
    node_network_receive_multicast_total{device="br-c4f03cf75ed7"} 0
    node_network_receive_multicast_total{device="docker0"} 0
    node_network_receive_multicast_total{device="lo"} 0
    node_network_receive_multicast_total{device="wlp170s0"} 0
    # HELP node_network_receive_nohandler_total Network device statistic receive_nohandler.
    # TYPE node_network_receive_nohandler_total counter
    node_network_receive_nohandler_total{device="br-c4f03cf75ed7"} 0
    node_network_receive_nohandler_total{device="docker0"} 0
    node_network_receive_nohandler_total{device="lo"} 0
    node_network_receive_nohandler_total{device="wlp170s0"} 0
    # HELP node_network_receive_packets_total Network device statistic receive_packets.
    # TYPE node_network_receive_packets_total counter
    node_network_receive_packets_total{device="br-c4f03cf75ed7"} 0
    node_network_receive_packets_total{device="docker0"} 0
    node_network_receive_packets_total{device="lo"} 2491
    node_network_receive_packets_total{device="wlp170s0"} 1.291775e+06
    # HELP node_network_speed_bytes Network device property: speed_bytes
    # TYPE node_network_speed_bytes gauge
    node_network_speed_bytes{device="br-c4f03cf75ed7"} -125000
    node_network_speed_bytes{device="docker0"} -125000
    # HELP node_network_transmit_bytes_total Network device statistic transmit_bytes.
    # TYPE node_network_transmit_bytes_total counter
    node_network_transmit_bytes_total{device="br-c4f03cf75ed7"} 0
    node_network_transmit_bytes_total{device="docker0"} 3038
    node_network_transmit_bytes_total{device="lo"} 751268
    node_network_transmit_bytes_total{device="wlp170s0"} 3.6595304e+07
    # HELP node_network_transmit_carrier_total Network device statistic transmit_carrier.
    # TYPE node_network_transmit_carrier_total counter
    node_network_transmit_carrier_total{device="br-c4f03cf75ed7"} 0
    node_network_transmit_carrier_total{device="docker0"} 0
    node_network_transmit_carrier_total{device="lo"} 0
    node_network_transmit_carrier_total{device="wlp170s0"} 0
    # HELP node_network_transmit_colls_total Network device statistic transmit_colls.
    # TYPE node_network_transmit_colls_total counter
    node_network_transmit_colls_total{device="br-c4f03cf75ed7"} 0
    node_network_transmit_colls_total{device="docker0"} 0
    node_network_transmit_colls_total{device="lo"} 0
    node_network_transmit_colls_total{device="wlp170s0"} 0
    # HELP node_network_transmit_compressed_total Network device statistic transmit_compressed.
    # TYPE node_network_transmit_compressed_total counter
    node_network_transmit_compressed_total{device="br-c4f03cf75ed7"} 0
    node_network_transmit_compressed_total{device="docker0"} 0
    node_network_transmit_compressed_total{device="lo"} 0
    node_network_transmit_compressed_total{device="wlp170s0"} 0
    # HELP node_network_transmit_drop_total Network device statistic transmit_drop.
    # TYPE node_network_transmit_drop_total counter
    node_network_transmit_drop_total{device="br-c4f03cf75ed7"} 0
    node_network_transmit_drop_total{device="docker0"} 0
    node_network_transmit_drop_total{device="lo"} 0
    node_network_transmit_drop_total{device="wlp170s0"} 0
    # HELP node_network_transmit_errs_total Network device statistic transmit_errs.
    # TYPE node_network_transmit_errs_total counter
    node_network_transmit_errs_total{device="br-c4f03cf75ed7"} 0
    node_network_transmit_errs_total{device="docker0"} 0
    node_network_transmit_errs_total{device="lo"} 0
    node_network_transmit_errs_total{device="wlp170s0"} 0
    # HELP node_network_transmit_fifo_total Network device statistic transmit_fifo.
    # TYPE node_network_transmit_fifo_total counter
    node_network_transmit_fifo_total{device="br-c4f03cf75ed7"} 0
    node_network_transmit_fifo_total{device="docker0"} 0
    node_network_transmit_fifo_total{device="lo"} 0
    node_network_transmit_fifo_total{device="wlp170s0"} 0
    # HELP node_network_transmit_packets_total Network device statistic transmit_packets.
    # TYPE node_network_transmit_packets_total counter
    node_network_transmit_packets_total{device="br-c4f03cf75ed7"} 0
    node_network_transmit_packets_total{device="docker0"} 41
    node_network_transmit_packets_total{device="lo"} 2491
    node_network_transmit_packets_total{device="wlp170s0"} 367244
    # HELP node_network_transmit_queue_length Network device property: transmit_queue_length
    # TYPE node_network_transmit_queue_length gauge
    node_network_transmit_queue_length{device="br-c4f03cf75ed7"} 0
    node_network_transmit_queue_length{device="docker0"} 0
    node_network_transmit_queue_length{device="lo"} 1000
    node_network_transmit_queue_length{device="wlp170s0"} 1000
    # HELP node_network_up Value is 1 if operstate is 'up', 0 otherwise.
    # TYPE node_network_up gauge
    node_network_up{device="br-c4f03cf75ed7"} 0
    node_network_up{device="docker0"} 0
    node_network_up{device="lo"} 0
    node_network_up{device="wlp170s0"} 1
    # HELP node_nf_conntrack_entries Number of currently allocated flow entries for connection tracking.
    # TYPE node_nf_conntrack_entries gauge
    node_nf_conntrack_entries 7
    # HELP node_nf_conntrack_entries_limit Maximum size of connection tracking table.
    # TYPE node_nf_conntrack_entries_limit gauge
    node_nf_conntrack_entries_limit 262144
    # HELP node_nvme_info Non-numeric data from /sys/class/nvme/<device>, value is always 1.
    # TYPE node_nvme_info gauge
    node_nvme_info{device="nvme0",firmware_revision="112000WD",model="WD_BLACK  SN750 2TB",serial="212348800873",state="live"} 1
    # HELP node_os_info A metric with a constant '1' value labeled by build_id, id, id_like, image_id, image_version, name, pretty_name, variant, variant_id, version, version_codename, version_id.
    # TYPE node_os_info gauge
    node_os_info{build_id="24.05.20231224.5f64a12",id="nixos",id_like="",image_id="",image_version="",name="NixOS",pretty_name="NixOS 24.05 (Uakari)",variant="",variant_id="",version="24.05 (Uakari)",version_codename="uakari",version_id="24.05"} 1
    # HELP node_os_version Metric containing the major.minor part of the OS version.
    # TYPE node_os_version gauge
    node_os_version{id="nixos",id_like="",name="NixOS"} 24.05
    # HELP node_power_supply_capacity capacity value of /sys/class/power_supply/<power_supply>.
    # TYPE node_power_supply_capacity gauge
    node_power_supply_capacity{power_supply="BAT1"} 35
    # HELP node_power_supply_charge_ampere charge_ampere value of /sys/class/power_supply/<power_supply>.
    # TYPE node_power_supply_charge_ampere gauge
    node_power_supply_charge_ampere{power_supply="BAT1"} 1.104
    # HELP node_power_supply_charge_full charge_full value of /sys/class/power_supply/<power_supply>.
    # TYPE node_power_supply_charge_full gauge
    node_power_supply_charge_full{power_supply="BAT1"} 3.114
    # HELP node_power_supply_charge_full_design charge_full_design value of /sys/class/power_supply/<power_supply>.
    # TYPE node_power_supply_charge_full_design gauge
    node_power_supply_charge_full_design{power_supply="BAT1"} 3.572
    # HELP node_power_supply_current_ampere current_ampere value of /sys/class/power_supply/<power_supply>.
    # TYPE node_power_supply_current_ampere gauge
    node_power_supply_current_ampere{power_supply="BAT1"} 2.814
    # HELP node_power_supply_cyclecount cyclecount value of /sys/class/power_supply/<power_supply>.
    # TYPE node_power_supply_cyclecount gauge
    node_power_supply_cyclecount{power_supply="BAT1"} 136
    # HELP node_power_supply_info info of /sys/class/power_supply/<power_supply>.
    # TYPE node_power_supply_info gauge
    node_power_supply_info{power_supply="ACAD",type="Mains"} 1
    node_power_supply_info{capacity_level="Normal",manufacturer="NVT",model_name="Framewo",power_supply="BAT1",serial_number="0037",status="Charging",technology="Li-ion",type="Battery"} 1
    # HELP node_power_supply_online online value of /sys/class/power_supply/<power_supply>.
    # TYPE node_power_supply_online gauge
    node_power_supply_online{power_supply="ACAD"} 1
    # HELP node_power_supply_present present value of /sys/class/power_supply/<power_supply>.
    # TYPE node_power_supply_present gauge
    node_power_supply_present{power_supply="BAT1"} 1
    # HELP node_power_supply_voltage_min_design voltage_min_design value of /sys/class/power_supply/<power_supply>.
    # TYPE node_power_supply_voltage_min_design gauge
    node_power_supply_voltage_min_design{power_supply="BAT1"} 15.4
    # HELP node_power_supply_voltage_volt voltage_volt value of /sys/class/power_supply/<power_supply>.
    # TYPE node_power_supply_voltage_volt gauge
    node_power_supply_voltage_volt{power_supply="BAT1"} 16.599
    # HELP node_pressure_cpu_waiting_seconds_total Total time in seconds that processes have waited for CPU time
    # TYPE node_pressure_cpu_waiting_seconds_total counter
    node_pressure_cpu_waiting_seconds_total 70.10262
    # HELP node_pressure_io_stalled_seconds_total Total time in seconds no process could make progress due to IO congestion
    # TYPE node_pressure_io_stalled_seconds_total counter
    node_pressure_io_stalled_seconds_total 11.080630999999999
    # HELP node_pressure_io_waiting_seconds_total Total time in seconds that processes have waited due to IO congestion
    # TYPE node_pressure_io_waiting_seconds_total counter
    node_pressure_io_waiting_seconds_total 12.054575999999999
    # HELP node_pressure_memory_stalled_seconds_total Total time in seconds no process could make progress due to memory congestion
    # TYPE node_pressure_memory_stalled_seconds_total counter
    node_pressure_memory_stalled_seconds_total 5.6e-05
    # HELP node_pressure_memory_waiting_seconds_total Total time in seconds that processes have waited for memory
    # TYPE node_pressure_memory_waiting_seconds_total counter
    node_pressure_memory_waiting_seconds_total 5.6e-05
    # HELP node_scrape_collector_duration_seconds node_exporter: Duration of a collector scrape.
    # TYPE node_scrape_collector_duration_seconds gauge
    node_scrape_collector_duration_seconds{collector="arp"} 8.5272e-05
    node_scrape_collector_duration_seconds{collector="bcache"} 2.818e-06
    node_scrape_collector_duration_seconds{collector="bonding"} 2.907e-06
    node_scrape_collector_duration_seconds{collector="btrfs"} 3.3368e-05
    node_scrape_collector_duration_seconds{collector="conntrack"} 2.0746e-05
    node_scrape_collector_duration_seconds{collector="cpu"} 0.000347794
    node_scrape_collector_duration_seconds{collector="cpufreq"} 0.005196606
    node_scrape_collector_duration_seconds{collector="diskstats"} 0.000190999
    node_scrape_collector_duration_seconds{collector="dmi"} 3.282e-06
    node_scrape_collector_duration_seconds{collector="edac"} 7.693e-06
    node_scrape_collector_duration_seconds{collector="entropy"} 2.4061e-05
    node_scrape_collector_duration_seconds{collector="fibrechannel"} 2.173e-06
    node_scrape_collector_duration_seconds{collector="filefd"} 1.6638e-05
    node_scrape_collector_duration_seconds{collector="filesystem"} 0.004307801
    node_scrape_collector_duration_seconds{collector="hwmon"} 0.051556393
    node_scrape_collector_duration_seconds{collector="infiniband"} 2.278e-06
    node_scrape_collector_duration_seconds{collector="ipvs"} 3.802e-06
    node_scrape_collector_duration_seconds{collector="loadavg"} 1.4715e-05
    node_scrape_collector_duration_seconds{collector="mdadm"} 4.032e-06
    node_scrape_collector_duration_seconds{collector="meminfo"} 7.9844e-05
    node_scrape_collector_duration_seconds{collector="netclass"} 0.000850065
    node_scrape_collector_duration_seconds{collector="netdev"} 0.000170992
    node_scrape_collector_duration_seconds{collector="netstat"} 0.000364095
    node_scrape_collector_duration_seconds{collector="nfs"} 5.449e-06
    node_scrape_collector_duration_seconds{collector="nfsd"} 8.37e-06
    node_scrape_collector_duration_seconds{collector="nvme"} 4.4501e-05
    node_scrape_collector_duration_seconds{collector="os"} 1.4172e-05
    node_scrape_collector_duration_seconds{collector="powersupplyclass"} 0.005860878
    node_scrape_collector_duration_seconds{collector="pressure"} 4.6612e-05
    node_scrape_collector_duration_seconds{collector="rapl"} 9.5073e-05
    node_scrape_collector_duration_seconds{collector="schedstat"} 2.913e-06
    node_scrape_collector_duration_seconds{collector="selinux"} 8.91e-07
    node_scrape_collector_duration_seconds{collector="sockstat"} 6.8526e-05
    node_scrape_collector_duration_seconds{collector="softnet"} 4.029e-05
    node_scrape_collector_duration_seconds{collector="stat"} 0.000123947
    node_scrape_collector_duration_seconds{collector="tapestats"} 5.658e-06
    node_scrape_collector_duration_seconds{collector="textfile"} 6.795e-06
    node_scrape_collector_duration_seconds{collector="thermal_zone"} 0.005182834
    node_scrape_collector_duration_seconds{collector="time"} 5.4105e-05
    node_scrape_collector_duration_seconds{collector="timex"} 4.342e-06
    node_scrape_collector_duration_seconds{collector="udp_queues"} 0.000147346
    node_scrape_collector_duration_seconds{collector="uname"} 2.858e-06
    node_scrape_collector_duration_seconds{collector="vmstat"} 6.4468e-05
    node_scrape_collector_duration_seconds{collector="xfs"} 0.00010323
    node_scrape_collector_duration_seconds{collector="zfs"} 1.1196e-05
    # HELP node_scrape_collector_success node_exporter: Whether a collector succeeded.
    # TYPE node_scrape_collector_success gauge
    node_scrape_collector_success{collector="arp"} 1
    node_scrape_collector_success{collector="bcache"} 1
    node_scrape_collector_success{collector="bonding"} 0
    node_scrape_collector_success{collector="btrfs"} 1
    node_scrape_collector_success{collector="conntrack"} 0
    node_scrape_collector_success{collector="cpu"} 1
    node_scrape_collector_success{collector="cpufreq"} 1
    node_scrape_collector_success{collector="diskstats"} 1
    node_scrape_collector_success{collector="dmi"} 1
    node_scrape_collector_success{collector="edac"} 1
    node_scrape_collector_success{collector="entropy"} 1
    node_scrape_collector_success{collector="fibrechannel"} 0
    node_scrape_collector_success{collector="filefd"} 1
    node_scrape_collector_success{collector="filesystem"} 1
    node_scrape_collector_success{collector="hwmon"} 1
    node_scrape_collector_success{collector="infiniband"} 0
    node_scrape_collector_success{collector="ipvs"} 0
    node_scrape_collector_success{collector="loadavg"} 1
    node_scrape_collector_success{collector="mdadm"} 0
    node_scrape_collector_success{collector="meminfo"} 1
    node_scrape_collector_success{collector="netclass"} 1
    node_scrape_collector_success{collector="netdev"} 1
    node_scrape_collector_success{collector="netstat"} 1
    node_scrape_collector_success{collector="nfs"} 0
    node_scrape_collector_success{collector="nfsd"} 0
    node_scrape_collector_success{collector="nvme"} 1
    node_scrape_collector_success{collector="os"} 1
    node_scrape_collector_success{collector="powersupplyclass"} 1
    node_scrape_collector_success{collector="pressure"} 1
    node_scrape_collector_success{collector="rapl"} 0
    node_scrape_collector_success{collector="schedstat"} 0
    node_scrape_collector_success{collector="selinux"} 1
    node_scrape_collector_success{collector="sockstat"} 1
    node_scrape_collector_success{collector="softnet"} 1
    node_scrape_collector_success{collector="stat"} 1
    node_scrape_collector_success{collector="tapestats"} 0
    node_scrape_collector_success{collector="textfile"} 1
    node_scrape_collector_success{collector="thermal_zone"} 1
    node_scrape_collector_success{collector="time"} 1
    node_scrape_collector_success{collector="timex"} 1
    node_scrape_collector_success{collector="udp_queues"} 1
    node_scrape_collector_success{collector="uname"} 1
    node_scrape_collector_success{collector="vmstat"} 1
    node_scrape_collector_success{collector="xfs"} 1
    node_scrape_collector_success{collector="zfs"} 0
    # HELP node_selinux_enabled SELinux is enabled, 1 is true, 0 is false
    # TYPE node_selinux_enabled gauge
    node_selinux_enabled 0
    # HELP node_sockstat_FRAG6_inuse Number of FRAG6 sockets in state inuse.
    # TYPE node_sockstat_FRAG6_inuse gauge
    node_sockstat_FRAG6_inuse 0
    # HELP node_sockstat_FRAG6_memory Number of FRAG6 sockets in state memory.
    # TYPE node_sockstat_FRAG6_memory gauge
    node_sockstat_FRAG6_memory 0
    # HELP node_sockstat_FRAG_inuse Number of FRAG sockets in state inuse.
    # TYPE node_sockstat_FRAG_inuse gauge
    node_sockstat_FRAG_inuse 0
    # HELP node_sockstat_FRAG_memory Number of FRAG sockets in state memory.
    # TYPE node_sockstat_FRAG_memory gauge
    node_sockstat_FRAG_memory 0
    # HELP node_sockstat_RAW6_inuse Number of RAW6 sockets in state inuse.
    # TYPE node_sockstat_RAW6_inuse gauge
    node_sockstat_RAW6_inuse 1
    # HELP node_sockstat_RAW_inuse Number of RAW sockets in state inuse.
    # TYPE node_sockstat_RAW_inuse gauge
    node_sockstat_RAW_inuse 1
    # HELP node_sockstat_TCP6_inuse Number of TCP6 sockets in state inuse.
    # TYPE node_sockstat_TCP6_inuse gauge
    node_sockstat_TCP6_inuse 1
    # HELP node_sockstat_TCP_alloc Number of TCP sockets in state alloc.
    # TYPE node_sockstat_TCP_alloc gauge
    node_sockstat_TCP_alloc 12
    # HELP node_sockstat_TCP_inuse Number of TCP sockets in state inuse.
    # TYPE node_sockstat_TCP_inuse gauge
    node_sockstat_TCP_inuse 11
    # HELP node_sockstat_TCP_mem Number of TCP sockets in state mem.
    # TYPE node_sockstat_TCP_mem gauge
    node_sockstat_TCP_mem 313
    # HELP node_sockstat_TCP_mem_bytes Number of TCP sockets in state mem_bytes.
    # TYPE node_sockstat_TCP_mem_bytes gauge
    node_sockstat_TCP_mem_bytes 1.282048e+06
    # HELP node_sockstat_TCP_orphan Number of TCP sockets in state orphan.
    # TYPE node_sockstat_TCP_orphan gauge
    node_sockstat_TCP_orphan 0
    # HELP node_sockstat_TCP_tw Number of TCP sockets in state tw.
    # TYPE node_sockstat_TCP_tw gauge
    node_sockstat_TCP_tw 2
    # HELP node_sockstat_UDP6_inuse Number of UDP6 sockets in state inuse.
    # TYPE node_sockstat_UDP6_inuse gauge
    node_sockstat_UDP6_inuse 3
    # HELP node_sockstat_UDPLITE6_inuse Number of UDPLITE6 sockets in state inuse.
    # TYPE node_sockstat_UDPLITE6_inuse gauge
    node_sockstat_UDPLITE6_inuse 0
    # HELP node_sockstat_UDPLITE_inuse Number of UDPLITE sockets in state inuse.
    # TYPE node_sockstat_UDPLITE_inuse gauge
    node_sockstat_UDPLITE_inuse 0
    # HELP node_sockstat_UDP_inuse Number of UDP sockets in state inuse.
    # TYPE node_sockstat_UDP_inuse gauge
    node_sockstat_UDP_inuse 10
    # HELP node_sockstat_UDP_mem Number of UDP sockets in state mem.
    # TYPE node_sockstat_UDP_mem gauge
    node_sockstat_UDP_mem 200
    # HELP node_sockstat_UDP_mem_bytes Number of UDP sockets in state mem_bytes.
    # TYPE node_sockstat_UDP_mem_bytes gauge
    node_sockstat_UDP_mem_bytes 819200
    # HELP node_sockstat_sockets_used Number of IPv4 sockets in use.
    # TYPE node_sockstat_sockets_used gauge
    node_sockstat_sockets_used 621
    # HELP node_softnet_backlog_len Softnet backlog status
    # TYPE node_softnet_backlog_len gauge
    node_softnet_backlog_len{cpu="0"} 0
    node_softnet_backlog_len{cpu="1"} 0
    node_softnet_backlog_len{cpu="2"} 0
    node_softnet_backlog_len{cpu="3"} 0
    node_softnet_backlog_len{cpu="4"} 0
    node_softnet_backlog_len{cpu="5"} 0
    node_softnet_backlog_len{cpu="6"} 0
    node_softnet_backlog_len{cpu="7"} 0
    # HELP node_softnet_cpu_collision_total Number of collision occur while obtaining device lock while transmitting
    # TYPE node_softnet_cpu_collision_total counter
    node_softnet_cpu_collision_total{cpu="0"} 0
    node_softnet_cpu_collision_total{cpu="1"} 0
    node_softnet_cpu_collision_total{cpu="2"} 0
    node_softnet_cpu_collision_total{cpu="3"} 0
    node_softnet_cpu_collision_total{cpu="4"} 0
    node_softnet_cpu_collision_total{cpu="5"} 0
    node_softnet_cpu_collision_total{cpu="6"} 0
    node_softnet_cpu_collision_total{cpu="7"} 0
    # HELP node_softnet_dropped_total Number of dropped packets
    # TYPE node_softnet_dropped_total counter
    node_softnet_dropped_total{cpu="0"} 0
    node_softnet_dropped_total{cpu="1"} 0
    node_softnet_dropped_total{cpu="2"} 0
    node_softnet_dropped_total{cpu="3"} 0
    node_softnet_dropped_total{cpu="4"} 0
    node_softnet_dropped_total{cpu="5"} 0
    node_softnet_dropped_total{cpu="6"} 0
    node_softnet_dropped_total{cpu="7"} 0
    # HELP node_softnet_flow_limit_count_total Number of times flow limit has been reached
    # TYPE node_softnet_flow_limit_count_total counter
    node_softnet_flow_limit_count_total{cpu="0"} 0
    node_softnet_flow_limit_count_total{cpu="1"} 0
    node_softnet_flow_limit_count_total{cpu="2"} 0
    node_softnet_flow_limit_count_total{cpu="3"} 0
    node_softnet_flow_limit_count_total{cpu="4"} 0
    node_softnet_flow_limit_count_total{cpu="5"} 0
    node_softnet_flow_limit_count_total{cpu="6"} 0
    node_softnet_flow_limit_count_total{cpu="7"} 0
    # HELP node_softnet_processed_total Number of processed packets
    # TYPE node_softnet_processed_total counter
    node_softnet_processed_total{cpu="0"} 10579
    node_softnet_processed_total{cpu="1"} 104200
    node_softnet_processed_total{cpu="2"} 64829
    node_softnet_processed_total{cpu="3"} 33960
    node_softnet_processed_total{cpu="4"} 12842
    node_softnet_processed_total{cpu="5"} 72843
    node_softnet_processed_total{cpu="6"} 144709
    node_softnet_processed_total{cpu="7"} 38671
    # HELP node_softnet_received_rps_total Number of times cpu woken up received_rps
    # TYPE node_softnet_received_rps_total counter
    node_softnet_received_rps_total{cpu="0"} 0
    node_softnet_received_rps_total{cpu="1"} 0
    node_softnet_received_rps_total{cpu="2"} 0
    node_softnet_received_rps_total{cpu="3"} 0
    node_softnet_received_rps_total{cpu="4"} 0
    node_softnet_received_rps_total{cpu="5"} 0
    node_softnet_received_rps_total{cpu="6"} 0
    node_softnet_received_rps_total{cpu="7"} 0
    # HELP node_softnet_times_squeezed_total Number of times processing packets ran out of quota
    # TYPE node_softnet_times_squeezed_total counter
    node_softnet_times_squeezed_total{cpu="0"} 0
    node_softnet_times_squeezed_total{cpu="1"} 0
    node_softnet_times_squeezed_total{cpu="2"} 0
    node_softnet_times_squeezed_total{cpu="3"} 0
    node_softnet_times_squeezed_total{cpu="4"} 0
    node_softnet_times_squeezed_total{cpu="5"} 4
    node_softnet_times_squeezed_total{cpu="6"} 0
    node_softnet_times_squeezed_total{cpu="7"} 0
    # HELP node_textfile_scrape_error 1 if there was an error opening or reading a file, 0 otherwise
    # TYPE node_textfile_scrape_error gauge
    node_textfile_scrape_error 0
    # HELP node_thermal_zone_temp Zone temperature in Celsius
    # TYPE node_thermal_zone_temp gauge
    node_thermal_zone_temp{type="INT3400 Thermal",zone="0"} 20
    node_thermal_zone_temp{type="SEN3",zone="1"} 38.85
    node_thermal_zone_temp{type="SEN5",zone="3"} 40.85
    node_thermal_zone_temp{type="TCPU",zone="4"} 41.05
    node_thermal_zone_temp{type="acpitz",zone="2"} 41.8
    node_thermal_zone_temp{type="iwlwifi_1",zone="6"} 44
    node_thermal_zone_temp{type="x86_pkg_temp",zone="5"} 41
    # HELP node_time_clocksource_available_info Available clocksources read from '/sys/devices/system/clocksource'.
    # TYPE node_time_clocksource_available_info gauge
    node_time_clocksource_available_info{clocksource="acpi_pm",device="0"} 1
    node_time_clocksource_available_info{clocksource="tsc",device="0"} 1
    # HELP node_time_clocksource_current_info Current clocksource read from '/sys/devices/system/clocksource'.
    # TYPE node_time_clocksource_current_info gauge
    node_time_clocksource_current_info{clocksource="tsc",device="0"} 1
    # HELP node_time_seconds System time in seconds since epoch (1970).
    # TYPE node_time_seconds gauge
    node_time_seconds 1.7058557961414876e+09
    # HELP node_time_zone_offset_seconds System time zone offset in seconds.
    # TYPE node_time_zone_offset_seconds gauge
    node_time_zone_offset_seconds{time_zone="CET"} 3600
    # HELP node_timex_estimated_error_seconds Estimated error in seconds.
    # TYPE node_timex_estimated_error_seconds gauge
    node_timex_estimated_error_seconds 0
    # HELP node_timex_frequency_adjustment_ratio Local clock frequency adjustment.
    # TYPE node_timex_frequency_adjustment_ratio gauge
    node_timex_frequency_adjustment_ratio 0.9999887462310791
    # HELP node_timex_loop_time_constant Phase-locked loop time constant.
    # TYPE node_timex_loop_time_constant gauge
    node_timex_loop_time_constant 7
    # HELP node_timex_maxerror_seconds Maximum error in seconds.
    # TYPE node_timex_maxerror_seconds gauge
    node_timex_maxerror_seconds 1.311
    # HELP node_timex_offset_seconds Time offset in between local system and reference clock.
    # TYPE node_timex_offset_seconds gauge
    node_timex_offset_seconds 2.6371e-05
    # HELP node_timex_pps_calibration_total Pulse per second count of calibration intervals.
    # TYPE node_timex_pps_calibration_total counter
    node_timex_pps_calibration_total 0
    # HELP node_timex_pps_error_total Pulse per second count of calibration errors.
    # TYPE node_timex_pps_error_total counter
    node_timex_pps_error_total 0
    # HELP node_timex_pps_frequency_hertz Pulse per second frequency.
    # TYPE node_timex_pps_frequency_hertz gauge
    node_timex_pps_frequency_hertz 0
    # HELP node_timex_pps_jitter_seconds Pulse per second jitter.
    # TYPE node_timex_pps_jitter_seconds gauge
    node_timex_pps_jitter_seconds 0
    # HELP node_timex_pps_jitter_total Pulse per second count of jitter limit exceeded events.
    # TYPE node_timex_pps_jitter_total counter
    node_timex_pps_jitter_total 0
    # HELP node_timex_pps_shift_seconds Pulse per second interval duration.
    # TYPE node_timex_pps_shift_seconds gauge
    node_timex_pps_shift_seconds 0
    # HELP node_timex_pps_stability_exceeded_total Pulse per second count of stability limit exceeded events.
    # TYPE node_timex_pps_stability_exceeded_total counter
    node_timex_pps_stability_exceeded_total 0
    # HELP node_timex_pps_stability_hertz Pulse per second stability, average of recent frequency changes.
    # TYPE node_timex_pps_stability_hertz gauge
    node_timex_pps_stability_hertz 0
    # HELP node_timex_status Value of the status array bits.
    # TYPE node_timex_status gauge
    node_timex_status 8193
    # HELP node_timex_sync_status Is clock synchronized to a reliable server (1 = yes, 0 = no).
    # TYPE node_timex_sync_status gauge
    node_timex_sync_status 1
    # HELP node_timex_tai_offset_seconds International Atomic Time (TAI) offset.
    # TYPE node_timex_tai_offset_seconds gauge
    node_timex_tai_offset_seconds 0
    # HELP node_timex_tick_seconds Seconds between clock ticks.
    # TYPE node_timex_tick_seconds gauge
    node_timex_tick_seconds 0.01
    # HELP node_udp_queues Number of allocated memory in the kernel for UDP datagrams in bytes.
    # TYPE node_udp_queues gauge
    node_udp_queues{ip="v4",queue="rx"} 0
    node_udp_queues{ip="v4",queue="tx"} 0
    node_udp_queues{ip="v6",queue="rx"} 0
    node_udp_queues{ip="v6",queue="tx"} 0
    # HELP node_uname_info Labeled system information as provided by the uname system call.
    # TYPE node_uname_info gauge
    node_uname_info{domainname="(none)",machine="x86_64",nodename="rilke",release="6.1.69",sysname="Linux",version="#1-NixOS SMP PREEMPT_DYNAMIC Wed Dec 20 16:00:29 UTC 2023"} 1
    # HELP node_vmstat_oom_kill /proc/vmstat information field oom_kill.
    # TYPE node_vmstat_oom_kill untyped
    node_vmstat_oom_kill 0
    # HELP node_vmstat_pgfault /proc/vmstat information field pgfault.
    # TYPE node_vmstat_pgfault untyped
    node_vmstat_pgfault 9.9110578e+07
    # HELP node_vmstat_pgmajfault /proc/vmstat information field pgmajfault.
    # TYPE node_vmstat_pgmajfault untyped
    node_vmstat_pgmajfault 16420
    # HELP node_vmstat_pgpgin /proc/vmstat information field pgpgin.
    # TYPE node_vmstat_pgpgin untyped
    node_vmstat_pgpgin 2.595487e+06
    # HELP node_vmstat_pgpgout /proc/vmstat information field pgpgout.
    # TYPE node_vmstat_pgpgout untyped
    node_vmstat_pgpgout 1.5018674e+07
    # HELP node_vmstat_pswpin /proc/vmstat information field pswpin.
    # TYPE node_vmstat_pswpin untyped
    node_vmstat_pswpin 0
    # HELP node_vmstat_pswpout /proc/vmstat information field pswpout.
    # TYPE node_vmstat_pswpout untyped
    node_vmstat_pswpout 0
    # HELP node_xfs_allocation_btree_compares_total Number of allocation B-tree compares for a filesystem.
    # TYPE node_xfs_allocation_btree_compares_total counter
    node_xfs_allocation_btree_compares_total{device="dm-2"} 0
    # HELP node_xfs_allocation_btree_lookups_total Number of allocation B-tree lookups for a filesystem.
    # TYPE node_xfs_allocation_btree_lookups_total counter
    node_xfs_allocation_btree_lookups_total{device="dm-2"} 0
    # HELP node_xfs_allocation_btree_records_deleted_total Number of allocation B-tree records deleted for a filesystem.
    # TYPE node_xfs_allocation_btree_records_deleted_total counter
    node_xfs_allocation_btree_records_deleted_total{device="dm-2"} 0
    # HELP node_xfs_allocation_btree_records_inserted_total Number of allocation B-tree records inserted for a filesystem.
    # TYPE node_xfs_allocation_btree_records_inserted_total counter
    node_xfs_allocation_btree_records_inserted_total{device="dm-2"} 0
    # HELP node_xfs_block_map_btree_compares_total Number of block map B-tree compares for a filesystem.
    # TYPE node_xfs_block_map_btree_compares_total counter
    node_xfs_block_map_btree_compares_total{device="dm-2"} 0
    # HELP node_xfs_block_map_btree_lookups_total Number of block map B-tree lookups for a filesystem.
    # TYPE node_xfs_block_map_btree_lookups_total counter
    node_xfs_block_map_btree_lookups_total{device="dm-2"} 0
    # HELP node_xfs_block_map_btree_records_deleted_total Number of block map B-tree records deleted for a filesystem.
    # TYPE node_xfs_block_map_btree_records_deleted_total counter
    node_xfs_block_map_btree_records_deleted_total{device="dm-2"} 0
    # HELP node_xfs_block_map_btree_records_inserted_total Number of block map B-tree records inserted for a filesystem.
    # TYPE node_xfs_block_map_btree_records_inserted_total counter
    node_xfs_block_map_btree_records_inserted_total{device="dm-2"} 0
    # HELP node_xfs_block_mapping_extent_list_compares_total Number of extent list compares for a filesystem.
    # TYPE node_xfs_block_mapping_extent_list_compares_total counter
    node_xfs_block_mapping_extent_list_compares_total{device="dm-2"} 0
    # HELP node_xfs_block_mapping_extent_list_deletions_total Number of extent list deletions for a filesystem.
    # TYPE node_xfs_block_mapping_extent_list_deletions_total counter
    node_xfs_block_mapping_extent_list_deletions_total{device="dm-2"} 134713
    # HELP node_xfs_block_mapping_extent_list_insertions_total Number of extent list insertions for a filesystem.
    # TYPE node_xfs_block_mapping_extent_list_insertions_total counter
    node_xfs_block_mapping_extent_list_insertions_total{device="dm-2"} 278126
    # HELP node_xfs_block_mapping_extent_list_lookups_total Number of extent list lookups for a filesystem.
    # TYPE node_xfs_block_mapping_extent_list_lookups_total counter
    node_xfs_block_mapping_extent_list_lookups_total{device="dm-2"} 8.107787e+06
    # HELP node_xfs_block_mapping_reads_total Number of block map for read operations for a filesystem.
    # TYPE node_xfs_block_mapping_reads_total counter
    node_xfs_block_mapping_reads_total{device="dm-2"} 3.598725e+06
    # HELP node_xfs_block_mapping_unmaps_total Number of block unmaps (deletes) for a filesystem.
    # TYPE node_xfs_block_mapping_unmaps_total counter
    node_xfs_block_mapping_unmaps_total{device="dm-2"} 309034
    # HELP node_xfs_block_mapping_writes_total Number of block map for write operations for a filesystem.
    # TYPE node_xfs_block_mapping_writes_total counter
    node_xfs_block_mapping_writes_total{device="dm-2"} 3.859576e+06
    # HELP node_xfs_directory_operation_create_total Number of times a new directory entry was created for a filesystem.
    # TYPE node_xfs_directory_operation_create_total counter
    node_xfs_directory_operation_create_total{device="dm-2"} 294675
    # HELP node_xfs_directory_operation_getdents_total Number of times the directory getdents operation was performed for a filesystem.
    # TYPE node_xfs_directory_operation_getdents_total counter
    node_xfs_directory_operation_getdents_total{device="dm-2"} 2.037409e+06
    # HELP node_xfs_directory_operation_lookup_total Number of file name directory lookups which miss the operating systems directory name lookup cache.
    # TYPE node_xfs_directory_operation_lookup_total counter
    node_xfs_directory_operation_lookup_total{device="dm-2"} 404849
    # HELP node_xfs_directory_operation_remove_total Number of times an existing directory entry was created for a filesystem.
    # TYPE node_xfs_directory_operation_remove_total counter
    node_xfs_directory_operation_remove_total{device="dm-2"} 146095
    # HELP node_xfs_extent_allocation_blocks_allocated_total Number of blocks allocated for a filesystem.
    # TYPE node_xfs_extent_allocation_blocks_allocated_total counter
    node_xfs_extent_allocation_blocks_allocated_total{device="dm-2"} 3.82397e+06
    # HELP node_xfs_extent_allocation_blocks_freed_total Number of blocks freed for a filesystem.
    # TYPE node_xfs_extent_allocation_blocks_freed_total counter
    node_xfs_extent_allocation_blocks_freed_total{device="dm-2"} 1.761409e+06
    # HELP node_xfs_extent_allocation_extents_allocated_total Number of extents allocated for a filesystem.
    # TYPE node_xfs_extent_allocation_extents_allocated_total counter
    node_xfs_extent_allocation_extents_allocated_total{device="dm-2"} 142466
    # HELP node_xfs_extent_allocation_extents_freed_total Number of extents freed for a filesystem.
    # TYPE node_xfs_extent_allocation_extents_freed_total counter
    node_xfs_extent_allocation_extents_freed_total{device="dm-2"} 73366
    # HELP node_xfs_inode_operation_attempts_total Number of times the OS looked for an XFS inode in the inode cache.
    # TYPE node_xfs_inode_operation_attempts_total counter
    node_xfs_inode_operation_attempts_total{device="dm-2"} 311983
    # HELP node_xfs_inode_operation_attribute_changes_total Number of times the OS explicitly changed the attributes of an XFS inode.
    # TYPE node_xfs_inode_operation_attribute_changes_total counter
    node_xfs_inode_operation_attribute_changes_total{device="dm-2"} 388809
    # HELP node_xfs_inode_operation_duplicates_total Number of times the OS tried to add a missing XFS inode to the inode cache, but found it had already been added by another process.
    # TYPE node_xfs_inode_operation_duplicates_total counter
    node_xfs_inode_operation_duplicates_total{device="dm-2"} 0
    # HELP node_xfs_inode_operation_found_total Number of times the OS looked for and found an XFS inode in the inode cache.
    # TYPE node_xfs_inode_operation_found_total counter
    node_xfs_inode_operation_found_total{device="dm-2"} 22249
    # HELP node_xfs_inode_operation_missed_total Number of times the OS looked for an XFS inode in the cache, but did not find it.
    # TYPE node_xfs_inode_operation_missed_total counter
    node_xfs_inode_operation_missed_total{device="dm-2"} 289734
    # HELP node_xfs_inode_operation_reclaims_total Number of times the OS reclaimed an XFS inode from the inode cache to free memory for another purpose.
    # TYPE node_xfs_inode_operation_reclaims_total counter
    node_xfs_inode_operation_reclaims_total{device="dm-2"} 110006
    # HELP node_xfs_inode_operation_recycled_total Number of times the OS found an XFS inode in the cache, but could not use it as it was being recycled.
    # TYPE node_xfs_inode_operation_recycled_total counter
    node_xfs_inode_operation_recycled_total{device="dm-2"} 0
    # HELP node_xfs_read_calls_total Number of read(2) system calls made to files in a filesystem.
    # TYPE node_xfs_read_calls_total counter
    node_xfs_read_calls_total{device="dm-2"} 7.805322e+06
    # HELP node_xfs_vnode_active_total Number of vnodes not on free lists for a filesystem.
    # TYPE node_xfs_vnode_active_total counter
    node_xfs_vnode_active_total{device="dm-2"} 179728
    # HELP node_xfs_vnode_allocate_total Number of times vn_alloc called for a filesystem.
    # TYPE node_xfs_vnode_allocate_total counter
    node_xfs_vnode_allocate_total{device="dm-2"} 0
    # HELP node_xfs_vnode_get_total Number of times vn_get called for a filesystem.
    # TYPE node_xfs_vnode_get_total counter
    node_xfs_vnode_get_total{device="dm-2"} 0
    # HELP node_xfs_vnode_hold_total Number of times vn_hold called for a filesystem.
    # TYPE node_xfs_vnode_hold_total counter
    node_xfs_vnode_hold_total{device="dm-2"} 0
    # HELP node_xfs_vnode_reclaim_total Number of times vn_reclaim called for a filesystem.
    # TYPE node_xfs_vnode_reclaim_total counter
    node_xfs_vnode_reclaim_total{device="dm-2"} 130782
    # HELP node_xfs_vnode_release_total Number of times vn_rele called for a filesystem.
    # TYPE node_xfs_vnode_release_total counter
    node_xfs_vnode_release_total{device="dm-2"} 130782
    # HELP node_xfs_vnode_remove_total Number of times vn_remove called for a filesystem.
    # TYPE node_xfs_vnode_remove_total counter
    node_xfs_vnode_remove_total{device="dm-2"} 130782
    # HELP node_xfs_write_calls_total Number of write(2) system calls made to files in a filesystem.
    # TYPE node_xfs_write_calls_total counter
    node_xfs_write_calls_total{device="dm-2"} 3.622646e+06
    # HELP process_virtual_memory_max_bytes Maximum amount of virtual memory available in bytes.
    # TYPE process_virtual_memory_max_bytes gauge
    process_virtual_memory_max_bytes 1.8446744073709552e+19
    # HELP promhttp_metric_handler_errors_total Total number of internal errors encountered by the promhttp metric handler.
    # TYPE promhttp_metric_handler_errors_total counter
    promhttp_metric_handler_errors_total{cause="encoding"} 0
    promhttp_metric_handler_errors_total{cause="gathering"} 0
    # HELP promhttp_metric_handler_requests_in_flight Current number of scrapes being served.
    # TYPE promhttp_metric_handler_requests_in_flight gauge
    promhttp_metric_handler_requests_in_flight 1
    # HELP promhttp_metric_handler_requests_total Total number of scrapes by HTTP status code.
    # TYPE promhttp_metric_handler_requests_total counter
    promhttp_metric_handler_requests_total{code="200"} 1
    promhttp_metric_handler_requests_total{code="500"} 0
    promhttp_metric_handler_requests_total{code="503"} 0

## Won't do

    # HELP go_gc_duration_seconds A summary of the pause duration of garbage collection cycles.
    # TYPE go_gc_duration_seconds summary
    go_gc_duration_seconds{quantile="0"} 1.2311e-05
    go_gc_duration_seconds{quantile="0.25"} 1.2311e-05
    go_gc_duration_seconds{quantile="0.5"} 1.2311e-05
    go_gc_duration_seconds{quantile="0.75"} 1.2311e-05
    go_gc_duration_seconds{quantile="1"} 1.2311e-05
    go_gc_duration_seconds_sum 1.2311e-05
    go_gc_duration_seconds_count 1
    # HELP go_goroutines Number of goroutines that currently exist.
    # TYPE go_goroutines gauge
    go_goroutines 7
    # HELP go_info Information about the Go environment.
    # TYPE go_info gauge
    go_info{version="go1.21.4"} 1
    # HELP go_memstats_alloc_bytes Number of bytes allocated and still in use.
    # TYPE go_memstats_alloc_bytes gauge
    go_memstats_alloc_bytes 1.583488e+06
    # HELP go_memstats_alloc_bytes_total Total number of bytes allocated, even if freed.
    # TYPE go_memstats_alloc_bytes_total counter
    go_memstats_alloc_bytes_total 3.805928e+06
    # HELP go_memstats_buck_hash_sys_bytes Number of bytes used by the profiling bucket hash table.
    # TYPE go_memstats_buck_hash_sys_bytes gauge
    go_memstats_buck_hash_sys_bytes 1.447279e+06
    # HELP go_memstats_frees_total Total number of frees.
    # TYPE go_memstats_frees_total counter
    go_memstats_frees_total 19221
    # HELP go_memstats_gc_sys_bytes Number of bytes used for garbage collection system metadata.
    # TYPE go_memstats_gc_sys_bytes gauge
    go_memstats_gc_sys_bytes 3.887776e+06
    # HELP go_memstats_heap_alloc_bytes Number of heap bytes allocated and still in use.
    # TYPE go_memstats_heap_alloc_bytes gauge
    go_memstats_heap_alloc_bytes 1.583488e+06
    # HELP go_memstats_heap_idle_bytes Number of heap bytes waiting to be used.
    # TYPE go_memstats_heap_idle_bytes gauge
    go_memstats_heap_idle_bytes 4.726784e+06
    # HELP go_memstats_heap_inuse_bytes Number of heap bytes that are in use.
    # TYPE go_memstats_heap_inuse_bytes gauge
    go_memstats_heap_inuse_bytes 3.170304e+06
    # HELP go_memstats_heap_objects Number of allocated objects.
    # TYPE go_memstats_heap_objects gauge
    go_memstats_heap_objects 17668
    # HELP go_memstats_heap_released_bytes Number of heap bytes released to OS.
    # TYPE go_memstats_heap_released_bytes gauge
    go_memstats_heap_released_bytes 3.145728e+06
    # HELP go_memstats_heap_sys_bytes Number of heap bytes obtained from system.
    # TYPE go_memstats_heap_sys_bytes gauge
    go_memstats_heap_sys_bytes 7.897088e+06
    # HELP go_memstats_last_gc_time_seconds Number of seconds since 1970 of last garbage collection.
    # TYPE go_memstats_last_gc_time_seconds gauge
    go_memstats_last_gc_time_seconds 1.7058557903574338e+09
    # HELP go_memstats_lookups_total Total number of pointer lookups.
    # TYPE go_memstats_lookups_total counter
    go_memstats_lookups_total 0
    # HELP go_memstats_mallocs_total Total number of mallocs.
    # TYPE go_memstats_mallocs_total counter
    go_memstats_mallocs_total 36889
    # HELP go_memstats_mcache_inuse_bytes Number of bytes in use by mcache structures.
    # TYPE go_memstats_mcache_inuse_bytes gauge
    go_memstats_mcache_inuse_bytes 1200
    # HELP go_memstats_mcache_sys_bytes Number of bytes used for mcache structures obtained from system.
    # TYPE go_memstats_mcache_sys_bytes gauge
    go_memstats_mcache_sys_bytes 15600
    # HELP go_memstats_mspan_inuse_bytes Number of bytes in use by mspan structures.
    # TYPE go_memstats_mspan_inuse_bytes gauge
    go_memstats_mspan_inuse_bytes 69216
    # HELP go_memstats_mspan_sys_bytes Number of bytes used for mspan structures obtained from system.
    # TYPE go_memstats_mspan_sys_bytes gauge
    go_memstats_mspan_sys_bytes 81480
    # HELP go_memstats_next_gc_bytes Number of heap bytes when next garbage collection will take place.
    # TYPE go_memstats_next_gc_bytes gauge
    go_memstats_next_gc_bytes 4.194304e+06
    # HELP go_memstats_other_sys_bytes Number of bytes used for other system allocations.
    # TYPE go_memstats_other_sys_bytes gauge
    go_memstats_other_sys_bytes 897473
    # HELP go_memstats_stack_inuse_bytes Number of bytes in use by the stack allocator.
    # TYPE go_memstats_stack_inuse_bytes gauge
    go_memstats_stack_inuse_bytes 491520
    # HELP go_memstats_stack_sys_bytes Number of bytes obtained from system for stack allocator.
    # TYPE go_memstats_stack_sys_bytes gauge
    go_memstats_stack_sys_bytes 491520
    # HELP go_memstats_sys_bytes Number of bytes obtained from system.
    # TYPE go_memstats_sys_bytes gauge
    go_memstats_sys_bytes 1.4718216e+07
    # HELP go_threads Number of OS threads created.
    # TYPE go_threads gauge
    go_threads 6

## Done

    # HELP node_boot_time_seconds Node boot time, in unixtime.
    # TYPE node_boot_time_seconds gauge
    node_boot_time_seconds 1.705836763e+09

    # HELP node_context_switches_total Total number of context switches.
    # TYPE node_context_switches_total counter
    node_context_switches_total 2.8640998e+07

    # HELP node_load1 1m load average.
    # TYPE node_load1 gauge
    node_load1 0.1

    # HELP node_load15 15m load average.
    # TYPE node_load15 gauge
    node_load15 0.34

    # HELP node_load5 5m load average.
    # TYPE node_load5 gauge
    node_load5 0.22

    # HELP node_memory_Active_anon_bytes Memory information field Active_anon_bytes.
    # TYPE node_memory_Active_anon_bytes gauge
    node_memory_Active_anon_bytes 3.478810624e+09

    # HELP node_memory_Active_bytes Memory information field Active_bytes.
    # TYPE node_memory_Active_bytes gauge
    node_memory_Active_bytes 5.303324672e+09

    # HELP node_memory_Active_file_bytes Memory information field Active_file_bytes.
    # TYPE node_memory_Active_file_bytes gauge
    node_memory_Active_file_bytes 1.824514048e+09

    # HELP node_memory_AnonHugePages_bytes Memory information field AnonHugePages_bytes.
    # TYPE node_memory_AnonHugePages_bytes gauge
    node_memory_AnonHugePages_bytes 0

    # HELP node_memory_AnonPages_bytes Memory information field AnonPages_bytes.
    # TYPE node_memory_AnonPages_bytes gauge
    node_memory_AnonPages_bytes 3.401265152e+09

    # HELP node_memory_Bounce_bytes Memory information field Bounce_bytes.
    # TYPE node_memory_Bounce_bytes gauge
    node_memory_Bounce_bytes 0

    # HELP node_memory_Buffers_bytes Memory information field Buffers_bytes.
    # TYPE node_memory_Buffers_bytes gauge
    node_memory_Buffers_bytes 2.88768e+06

    # HELP node_memory_Cached_bytes Memory information field Cached_bytes.
    # TYPE node_memory_Cached_bytes gauge
    node_memory_Cached_bytes 1.1580862464e+10

    # HELP node_memory_CmaFree_bytes Memory information field CmaFree_bytes.
    # TYPE node_memory_CmaFree_bytes gauge
    node_memory_CmaFree_bytes 0

    # HELP node_memory_CmaTotal_bytes Memory information field CmaTotal_bytes.
    # TYPE node_memory_CmaTotal_bytes gauge
    node_memory_CmaTotal_bytes 0

    # HELP node_memory_CommitLimit_bytes Memory information field CommitLimit_bytes.
    # TYPE node_memory_CommitLimit_bytes gauge
    node_memory_CommitLimit_bytes 3.7904789504e+10

    # HELP node_memory_Committed_AS_bytes Memory information field Committed_AS_bytes.
    # TYPE node_memory_Committed_AS_bytes gauge
    node_memory_Committed_AS_bytes 8.03995648e+09

    # HELP node_memory_DirectMap1G_bytes Memory information field DirectMap1G_bytes.
    # TYPE node_memory_DirectMap1G_bytes gauge
    node_memory_DirectMap1G_bytes 6.2277025792e+10

    # HELP node_memory_DirectMap2M_bytes Memory information field DirectMap2M_bytes.
    # TYPE node_memory_DirectMap2M_bytes gauge
    node_memory_DirectMap2M_bytes 6.002049024e+09

    # HELP node_memory_DirectMap4k_bytes Memory information field DirectMap4k_bytes.
    # TYPE node_memory_DirectMap4k_bytes gauge
    node_memory_DirectMap4k_bytes 2.13581824e+08

    # HELP node_memory_Dirty_bytes Memory information field Dirty_bytes.
    # TYPE node_memory_Dirty_bytes gauge
    node_memory_Dirty_bytes 0

    # HELP node_memory_FileHugePages_bytes Memory information field FileHugePages_bytes.
    # TYPE node_memory_FileHugePages_bytes gauge
    node_memory_FileHugePages_bytes 0

    # HELP node_memory_FilePmdMapped_bytes Memory information field FilePmdMapped_bytes.
    # TYPE node_memory_FilePmdMapped_bytes gauge
    node_memory_FilePmdMapped_bytes 0

    # HELP node_memory_HugePages_Free Memory information field HugePages_Free.
    # TYPE node_memory_HugePages_Free gauge
    node_memory_HugePages_Free 0

    # HELP node_memory_HugePages_Rsvd Memory information field HugePages_Rsvd.
    # TYPE node_memory_HugePages_Rsvd gauge
    node_memory_HugePages_Rsvd 0

    # HELP node_memory_HugePages_Surp Memory information field HugePages_Surp.
    # TYPE node_memory_HugePages_Surp gauge
    node_memory_HugePages_Surp 0

    # HELP node_memory_HugePages_Total Memory information field HugePages_Total.
    # TYPE node_memory_HugePages_Total gauge
    node_memory_HugePages_Total 0

    # HELP node_procs_blocked Number of processes blocked waiting for I/O to complete.
    # TYPE node_procs_blocked gauge
    node_procs_blocked 1

    # HELP node_procs_running Number of processes in runnable state.
    # TYPE node_procs_running gauge
    node_procs_running 2

    # HELP process_cpu_seconds_total Total user and system CPU time spent in seconds.
    # TYPE process_cpu_seconds_total counter
    process_cpu_seconds_total 0

    # HELP process_max_fds Maximum number of open file descriptors.
    # TYPE process_max_fds gauge
    process_max_fds 524288

    # HELP process_open_fds Number of open file descriptors.
    # TYPE process_open_fds gauge
    process_open_fds 9

    # HELP process_resident_memory_bytes Resident memory size in bytes.
    # TYPE process_resident_memory_bytes gauge
    process_resident_memory_bytes 1.7178624e+07

    # HELP process_start_time_seconds Start time of the process since unix epoch in seconds.
    # TYPE process_start_time_seconds gauge
    process_start_time_seconds 1.70585578135e+09

    # HELP process_virtual_memory_bytes Virtual memory size in bytes.
    # TYPE process_virtual_memory_bytes gauge
    process_virtual_memory_bytes 1.271386112e+09
