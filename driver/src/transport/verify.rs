use once_cell::sync::Lazy;
use reqwest_wasm::Client;
use kernel::error::KernelError;
use kernel::interfaces::transport::VerifierFlowTransporter;
use kernel::prelude::entities::Identifier;
use crate::error::DriverError;

pub struct VerifyFlowTransport {
    client: Client
}

impl VerifyFlowTransport {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait::async_trait]
impl VerifierFlowTransporter for VerifyFlowTransport {
    async fn verify(&self, ticket: &Identifier, otp: &str) -> Result<Identifier, KernelError> {
        let client = &self.client;
        let res = VerifyFlowHttpInternal::verify(ticket, otp, client).await?;
        Ok(res)
    }
}


pub(in crate::transport) struct VerifyFlowHttpInternal;

impl VerifyFlowHttpInternal {
    pub async fn verify(
        ticket: &Identifier,
        otp: &str,
        client: &Client
    ) -> Result<Identifier, DriverError> {
        let req = VerifyRequest {
            code: otp
        };
        let res = client.post(&*VERIFY_ENDPOINT)
            .query(&("ticket", ticket.as_ref()))
            .json(&req)
            .send()
            .await?
            .json::<VerifyResponse>()
            .await?;
        Ok(res.ticket)
    }
}


static VERIFY_ENDPOINT: Lazy<String> = Lazy::new(|| {
    dotenvy::var("VERIFY_ENDPOINT")
        .expect("Not set `VERIFY_ENDPOINT`!, This value required.")
});

#[derive(serde::Serialize)]
pub(in crate::transport) struct VerifyRequest<'a> {
    code: &'a str
}

#[derive(serde::Deserialize)]
pub(in crate::transport) struct VerifyResponse {
    ticket: Identifier
}