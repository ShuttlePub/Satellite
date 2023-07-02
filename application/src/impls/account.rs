use kernel::interfaces::transport::{DependOnSignUpFlowTransporter, DependOnVerifierFlowTransporter};
use crate::services::{CreateAccountService, PendingAccountService, VerifyAccountService};


// Default impl
impl<T> PendingAccountService for T
    where T: DependOnSignUpFlowTransporter {}


// Default impl
impl<T> VerifyAccountService for T
    where T: DependOnVerifierFlowTransporter {}


// Default impl
impl<T> CreateAccountService for T
    where T: DependOnSignUpFlowTransporter {}