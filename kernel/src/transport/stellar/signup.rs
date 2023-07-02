use crate::entities::{Account, Address, Identifier, Password, UserName};
use crate::error::KernelError;
use crate::transport::DependOnVerifierFlowTransporter;

#[orbital::export_service]
#[async_trait::async_trait]
pub trait SignUpFlowTransporter: 'static + Sync + Send
    + DependOnVerifierFlowTransporter
{
    async fn create_temp(&self, address: &Address) -> Result<Identifier, KernelError>;
    async fn create_main(&self, verified: &Identifier, name: &UserName, pass: &Password) -> Result<(), KernelError>;
}