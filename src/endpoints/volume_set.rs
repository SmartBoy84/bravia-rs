use restman_rs::{POST, endpoint};
use serde::Serialize;

use crate::{BraviaApi, BraviaReq, BraviaRes, client::BraviaServer, endpoints::Sony};

#[derive(Serialize)]
pub struct SetAudioVolume {
    pub target: String,
    pub volume: String,
}

impl BraviaApi for SetAudioVolume {
    const VERSION: &str = "1.2";
    const NAME: &str = "setAudioVolume";
    const ID: usize = 601; // example default ID
    type Endpoint = VolumeSet;
}

endpoint!(BraviaServer, pub VolumeSet, "audio", Sony, BraviaRes<()>, (), BraviaReq<SetAudioVolume>, POST);
