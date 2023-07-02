use kernel::interfaces::transport::{DependOnSignUpFlowTransporter, DependOnVerifierFlowTransporter, SignUpFlowTransporter, VerifierFlowTransporter};
use kernel::prelude::entities::{Account, Address, Identifier, Password, UserName};
use crate::error::ApplicationError;
use crate::transfer::account::{PendingAccountDto, VerifyAccountDto, CreateAccountDto};
use crate::transfer::identifier::IdentifierDto;

#[orbital::export_service]
#[async_trait::async_trait]
pub trait PendingAccountService: 'static + Sync + Send
    + DependOnSignUpFlowTransporter
{
    async fn pending(&self, pending: PendingAccountDto) -> Result<IdentifierDto, ApplicationError> {
        let PendingAccountDto { address, .. } = pending;
        let address = Address::new(address);
        let ticket = self.sign_up_flow_transporter()
            .create_temp(&address)
            .await?;
        Ok(ticket.into())
    }
}


#[orbital::export_service]
#[async_trait::async_trait]
pub trait VerifyAccountService: 'static + Sync + Send
    + DependOnVerifierFlowTransporter
{
    async fn verify(&self, verifier: VerifyAccountDto) -> Result<IdentifierDto, ApplicationError> {
        let VerifyAccountDto { ticket, mfa_code, .. } = verifier;
        let ticket = Identifier::new(ticket);
        let verified = self.verifier_flow_transporter()
            .verify(&ticket, &mfa_code)
            .await?;
        Ok(verified.into())
    }
}


#[orbital::export_service]
#[async_trait::async_trait]
pub trait CreateAccountService: 'static + Sync + Send
    + DependOnSignUpFlowTransporter
{
    async fn create(&self, create: CreateAccountDto) -> Result<(), ApplicationError> {
        let CreateAccountDto { ticket, name, pass, .. } = create;
        let ticket = Identifier::new(ticket);
        let name = UserName::new(name);
        let pass = Password::new(pass);
        self.sign_up_flow_transporter()
            .create_main(&ticket, &name, &pass)
            .await?;
        Ok(())
    }
}