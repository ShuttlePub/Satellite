#[derive(Debug, thiserror::Error)]
pub enum DriverError {
    #[error(transparent)]
    Reqwest(reqwest_wasm::Error),
    #[error(transparent)]
    Kernel(kernel::error::KernelError)
}

impl From<reqwest_wasm::Error> for DriverError {
    fn from(error: reqwest_wasm::Error) -> Self {
        Self::Reqwest(error)
    }
}

impl From<kernel::error::KernelError> for DriverError {
    fn from(error: kernel::error::KernelError) -> Self {
        Self::Kernel(error)
    }
}

impl From<DriverError> for kernel::error::KernelError {
    fn from(error: DriverError) -> Self {
        kernel::error::KernelError::Driver(anyhow::Error::new(error))
    }
}