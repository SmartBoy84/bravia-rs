pub mod client;
pub mod endpoints;

use restman_rs::request::{QueryPayload, SerialiseRequestPart, endpoints::Endpoint};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use thiserror::Error;

use crate::client::BraviaServer;

#[derive(Serialize)]
pub struct BraviaReq<T: BraviaApi> {
    method: String,
    id: usize,
    params: [T; 1],
    version: String,
}
impl<T: BraviaApi> QueryPayload for BraviaReq<T> {}

#[derive(Serialize)]
pub struct BraviaResInner<T: BraviaApi> {
    result: [T; 1],
    id: usize,
}

#[derive(Debug, Deserialize, Error)]
pub enum BraviaApiErr {
    #[error("empty res")]
    EmptyRes,
    #[error("api err")]
    ApiErr(BraviaResErr),
}

#[derive(Debug, Deserialize)]
pub struct BraviaResErr {
    code: u32,
    text: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum BraviaRes<T> {
    Success {
        result: Vec<T>,
        #[serde(skip)]
        id: (),
    },
    Error {
        error: (u32, String),
        #[serde(skip)]
        id: (),
    },
}

pub trait BraviaApi: Serialize + Sized {
    const NAME: &str;
    const VERSION: &str;
    const ID: usize;
    type Endpoint: Endpoint<Ser = BraviaServer, Payload = BraviaReq<Self>, Para = ()>
        + SerialiseRequestPart<()>;
}

pub trait BraviaApiResInner: DeserializeOwned {}

// sealed trait pattern - allows the user to set Res to ()
pub trait BraviaApiRes {}
impl<T: BraviaApiRes + DeserializeOwned> BraviaApiResInner for T {}

impl BraviaApiResInner for () {}

impl<T: BraviaApi> From<T> for BraviaReq<T> {
    fn from(value: T) -> Self {
        BraviaReq {
            method: T::NAME.to_owned(),
            id: T::ID,
            version: T::VERSION.to_owned(),
            params: [value],
        }
    }
}

// can overload functions! as long as you can prove trait bounds dont overlap
impl<T: BraviaApiRes> Into<Result<T, BraviaApiErr>> for BraviaRes<T> {
    fn into(self) -> Result<T, BraviaApiErr> {
        match self {
            BraviaRes::Success { result, .. } => {
                Ok(result.into_iter().next().ok_or(BraviaApiErr::EmptyRes)?)
            }
            BraviaRes::Error {
                error: (code, text),
                ..
            } => Err(BraviaApiErr::ApiErr(BraviaResErr { code, text })),
        }
    }
}

impl Into<Result<(), BraviaApiErr>> for BraviaRes<()> {
    fn into(self) -> Result<(), BraviaApiErr> {
        match self {
            BraviaRes::Success { .. } => Ok(()),
            BraviaRes::Error {
                error: (code, text),
                ..
            } => Err(BraviaApiErr::ApiErr(BraviaResErr { code, text })),
        }
    }
}
