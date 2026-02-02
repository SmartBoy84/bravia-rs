use restman_rs::{POST, endpoint};
use serde::Serialize;

use crate::{BraviaApi, BraviaReq, BraviaRes, client::BraviaServer, endpoints::System};

#[derive(Serialize)]
pub struct SetPowerStatus {
    pub status: bool,
}

impl BraviaApi for SetPowerStatus {
    const VERSION: &str = "1.0";
    const NAME: &str = "setPowerStatus";
    const ID: usize = 55;

    type Endpoint = PowerSet;
}

endpoint!(BraviaServer, pub PowerSet, "", System, BraviaRes<()>, (), BraviaReq<SetPowerStatus>, POST);
