use restman_rs::{POST, endpoint};
use serde::{Deserialize, Serialize};

use crate::{BraviaApi, BraviaApiRes, BraviaReq, BraviaRes, client::BraviaServer, endpoints::{Sony, System}};

#[derive(Debug, Deserialize)]
pub struct PowerStatusRes {
    pub status: String,
}

impl BraviaApiRes for PowerStatusRes {}

#[derive(Serialize)]
pub struct GetPowerStatus;

impl BraviaApi for GetPowerStatus {
    const VERSION: &str = "1.0";
    const NAME: &str = "getPowerStatus";
    const ID: usize = 50;
    type Endpoint = PowerSet;
}

endpoint!(BraviaServer, pub PowerSet, "", System, BraviaRes<PowerStatusRes>, (), BraviaReq<GetPowerStatus>, POST);
