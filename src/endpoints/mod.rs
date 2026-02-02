use restman_rs::request_part;

pub mod power_get;
pub mod power_set;
pub mod set_active;
pub mod volume_set;

// API has a single request part
request_part!(Sony, "sony", ()); // base

// there can be multiple endpoints pointing to one part (E.g., power get/set both point to system) so I made them all parts
request_part!(System, "system", Sony); // base
request_part!(AppControl, "appControl", Sony); // base
request_part!(Volume, "volume", Sony); // base
