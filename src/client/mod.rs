mod abstraction;

use restman_rs::{
    ApiBackendError, ApiHttpClient, DynamicServer, MethodMarkerGetter, Server,
    client::{AGENT, ApiClient, ApiClientBackend, ApiClientServer},
    request::{ApiPayload, ApiRequest, QueryPayload, ValidRequest, endpoints::Endpoint},
    ureq::UreqApiHttpClient,
};
use thiserror::Error;

use crate::{BraviaApi, BraviaApiErr, BraviaReq, BraviaRes};

const AUTH_HEADER_NAME: &str = "X-Auth-PSK";
const ROOT: &str = "sony";

#[derive(Error, Debug)]
pub enum BraviaError<C: ApiHttpClient> {
    #[error("backend err")]
    BackendError(#[from] ApiBackendError<C>),

    #[error("api error")]
    ApiError(#[from] BraviaApiErr),

    #[error("frontend err")]
    FrontEndErr(String), // I'm lazy - this is for debugging onmly - won't match on this
}
pub type BraviaResult<T, C> = Result<T, BraviaError<C>>;

pub struct BraviaServer {
    addr: String,
}

impl Server for BraviaServer {}
impl DynamicServer for BraviaServer {
    fn get_root(&self) -> &str {
        &self.addr
    }
}

pub fn make_payload<P: BraviaApi>(p: P) -> serde_json::Result<ApiPayload<BraviaReq<P>>>
where
    BraviaReq<P>: QueryPayload,
{
    ApiPayload::new(BraviaReq::from(p))
}

pub struct BraviaClient<T: ApiHttpClient = UreqApiHttpClient> {
    pass: String,
    server: BraviaServer,
    backend: T,
}

impl<C: ApiHttpClient> BraviaClient<C> {
    pub fn new_with_backend(backend: C, ip: &str, pass: &str) -> Self {
        Self {
            server: BraviaServer {
                addr: format!("http://{ip}"),
            },
            pass: pass.to_string(),
            backend,
        }
    }

    pub fn write_cmd<P, T>(&self, p: &ApiPayload<BraviaReq<P>>) -> BraviaResult<T, C>
    where
        P: BraviaApi<Endpoint: Endpoint<Res = BraviaRes<T>>>,
        <P::Endpoint as Endpoint>::Method: MethodMarkerGetter<C>,
        <P::Endpoint as Endpoint>::Res: Into<Result<T, BraviaApiErr>>,
    {
        let r = ApiRequest::<P::Endpoint>::new_with_server(&(), &self.server);
        Ok(self.send_payload(&r, p)?.into()?)
    }

    pub fn request<P, T>(&self, p: P) -> BraviaResult<T, C>
    where
        P: BraviaApi<Endpoint: Endpoint<Res = BraviaRes<T>>>,
        <P::Endpoint as Endpoint>::Method: MethodMarkerGetter<C>,
        <P::Endpoint as Endpoint>::Res: Into<Result<T, BraviaApiErr>>,
    {
        let payload = ApiPayload::new(BraviaReq::from(p)).unwrap();
        Ok(self.write_cmd(&payload)?)
    }
}

impl BraviaClient {
    pub fn new(pass: &str, ip: &str) -> Self {
        let mut backend = UreqApiHttpClient::new(AGENT);
        backend.set_authorisation_header_name(AUTH_HEADER_NAME);
        Self::new_with_backend(backend, ip, pass)
    }
}

impl<C: ApiHttpClient> ApiClientServer<BraviaServer> for BraviaClient<C> {}
impl<C: ApiHttpClient> ApiClientBackend<C> for BraviaClient<C> {
    fn backend(&self) -> &C {
        &self.backend
    }
    fn token(&self) -> &str {
        &self.pass
    }
}
