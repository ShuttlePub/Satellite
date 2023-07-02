use crate::entities::Identifier;
use crate::error::KernelError;

#[orbital::export_service]
#[async_trait::async_trait]
pub trait VerifierFlowTransporter: 'static + Sync + Send {
    async fn verify(&self, ticket: &Identifier, otp: &str) -> Result<Identifier, KernelError>;
}