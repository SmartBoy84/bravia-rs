use restman_rs::{POST, endpoint};
use serde::Serialize;

use crate::{BraviaApi, BraviaReq, BraviaRes, client::BraviaServer, endpoints::{AppControl, Sony}};

#[derive(Serialize)]
pub struct SetActiveApp {
    pub uri: String,
}

impl BraviaApi for SetActiveApp {
    const VERSION: &str = "1.0";
    const NAME: &str = "setActiveApp";
    const ID: usize = 601; // example default
    type Endpoint = VolumeSet;
}

endpoint!(BraviaServer, pub VolumeSet, "", AppControl, BraviaRes<()>, (), BraviaReq<SetActiveApp>, POST);
