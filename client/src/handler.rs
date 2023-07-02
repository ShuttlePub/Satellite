use application::services::{DependOnCreateAccountService, DependOnPendingAccountService, DependOnVerifyAccountService};
use driver::DriverInitializer;
use driver::transport::{SignupFlowTransport, VerifyFlowTransport};
use kernel::interfaces::transport::{DependOnSignUpFlowTransporter, DependOnVerifierFlowTransporter};

pub struct Handler {
    signup: SignupFlowTransport,
    verify: VerifyFlowTransport
}

impl Handler {
    pub fn init() -> Self {
        let http_client = DriverInitializer::http_client();
        let signup = SignupFlowTransport::new(http_client.clone());
        let verify = VerifyFlowTransport::new(http_client);
        Self {
            signup,
            verify
        }
    }
}

impl DependOnSignUpFlowTransporter for Handler {
    type SignUpFlowTransporter = SignupFlowTransport;
    fn sign_up_flow_transporter(&self) -> &Self::SignUpFlowTransporter {
        &self.signup
    }
}

impl DependOnVerifierFlowTransporter for Handler {
    type VerifierFlowTransporter = VerifyFlowTransport;
    fn verifier_flow_transporter(&self) -> &Self::VerifierFlowTransporter {
        &self.verify
    }
}

impl DependOnPendingAccountService for Handler {
    type PendingAccountService = Self;
    fn pending_account_service(&self) -> &Self::PendingAccountService {
        self
    }
}

impl DependOnCreateAccountService for Handler {
    type CreateAccountService = Self;
    fn create_account_service(&self) -> &Self::CreateAccountService {
        self
    }
}

impl DependOnVerifyAccountService for Handler {
    type VerifyAccountService = Self;
    fn verify_account_service(&self) -> &Self::VerifyAccountService {
        self
    }
}