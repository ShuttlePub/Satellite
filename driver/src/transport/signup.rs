use once_cell::sync::Lazy;
use reqwest_wasm::Client;
use kernel::error::KernelError;
use kernel::interfaces::transport::SignUpFlowTransporter;
use kernel::prelude::entities::{Address, Identifier, Password, UserName};
use crate::error::DriverError;

pub struct SignupFlowTransport {
    client: Client
}

impl SignupFlowTransport {
    #[allow(clippy::new_without_default)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait::async_trait]
impl SignUpFlowTransporter for SignupFlowTransport {
    async fn create_temp(&self, address: &Address) -> Result<Identifier, KernelError> {
        let client = &self.client;
        let res = SignupFlowHttpInternal::create_temp(address, client).await?;
        Ok(res)
    }

    async fn create_main(&self, verified: &Identifier, name: &UserName, pass: &Password) -> Result<(), KernelError> {
        let client = &self.client;
        let _res = SignupFlowHttpInternal::create_main(verified, name, pass, client).await?;
        Ok(())
    }
}


pub(in crate::transport) struct SignupFlowHttpInternal;

impl SignupFlowHttpInternal {
    pub async fn create_temp(
        address: &Address,
        client: &Client
    ) -> Result<Identifier, DriverError> {
        let req = SignupTemporalRequest {
            address: address.as_ref()
        };
        let res = client
            .post(&*SIGNUP_TEMP_ENDPOINT)
            .json(&req)
            .send()
            .await?
            .json::<IdentifierResponse>()
            .await?;
        Ok(res.ticket)
    }

    pub async fn create_main(
        verified: &Identifier,
        name: &UserName,
        pass: &Password,
        client: &Client
    ) -> Result<(), DriverError> {
        let req = SignupMainRequest {
            name: name.as_ref(),
            pass: pass.as_ref()
        };
        let _res = client
            .post(&*SIGNUP_TEMP_ENDPOINT)
            .query(&("ticket", verified.as_ref()))
            .json(&req)
            .send()
            .await?;
        Ok(())
    }
}

static SIGNUP_TEMP_ENDPOINT: Lazy<String> = Lazy::new(|| {
    dotenvy::var("SIGNUP_ENDPOINT")
        .expect("Not set `SIGNUP_TEMP_ENDPOINT`!, This value required.")
});

#[derive(Debug, serde::Serialize)]
pub(in crate::transport) struct SignupTemporalRequest<'a> {
    address: &'a str
}

#[derive(Debug, serde::Serialize)]
pub(in crate::transport) struct SignupMainRequest<'a> {
    name: &'a str,
    pass: &'a str
}

#[derive(Debug, serde::Deserialize)]
pub(in crate::transport) struct IdentifierResponse {
    ticket: Identifier
}


#[cfg(test)]
mod tests {
    use reqwest_wasm::Client;
    use kernel::prelude::entities::Address;
    use super::SignupFlowHttpInternal;

    #[tokio::test]
    #[ignore = "It was depend on stellar."]
    pub async fn create_temp_test() -> anyhow::Result<()> {
        let address = Address::new("example.user@example.com");
        let client = Client::new();
        let res = SignupFlowHttpInternal::create_temp(&address, &client).await?;
        println!("{:?}", res);
        Ok(())
    }
}