use lazy_static::lazy_static;
use prometheus::{register_int_gauge_vec, IntGaugeVec, Opts};

use crate::State;

lazy_static! {
    static ref DEVICE_MAPPER_INFO: IntGaugeVec = register_int_gauge_vec!(
        Opts::new(
            "node_disk_device_mapper_info",
            "Info about disk device mapper."
        ),
        &["device", "lv_layer", "lv_name", "name", "uuid", "vg_name"]
    )
    .unwrap();
}

pub fn update(_state: &State) {}
