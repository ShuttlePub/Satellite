#[derive(Debug)]
pub struct AccountDto {
    
}

#[derive(Debug)]
pub struct PendingAccountDto {
    pub address: String
}

#[derive(Debug)]
pub struct VerifyAccountDto {
    pub ticket: String,
    pub mfa_code: String
}

pub struct CreateAccountDto {
    pub ticket: String,
    pub name: String,
    pub pass: String
}