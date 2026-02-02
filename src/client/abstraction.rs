use restman_rs::{ApiHttpClient, Post};

use crate::{
    client::{BraviaClient, BraviaError, BraviaResult},
    endpoints::{
        power_get::GetPowerStatus, power_set::SetPowerStatus, set_active::SetActiveApp,
        volume_set::SetAudioVolume,
    },
};

impl<C: ApiHttpClient + Post> BraviaClient<C> {
    pub fn set_state(&self, status: bool) -> BraviaResult<(), C> {
        self.request(SetPowerStatus { status })
    }

    pub fn get_state(&self) -> BraviaResult<bool, C> {
        let s = self.request(GetPowerStatus)?.status;
        match s.as_str() {
            "active" => Ok(true),
            "standby" => Ok(false),
            _ => Err(BraviaError::FrontEndErr(format!("unexpected state: {}", s))),
        }
    }

    pub fn set_vol(&self, vol: usize) -> BraviaResult<(), C> {
        self.request(SetAudioVolume {
            target: "".to_string(),
            volume: vol.to_string(),
        })
    }

    pub fn set_active(&self, uri: &str) -> BraviaResult<(), C> {
        self.request(SetActiveApp { uri: uri.into() })
    }
}
