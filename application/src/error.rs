#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("An error occurred in kernel layer: {0}")]
    Kernel(kernel::error::KernelError)
}

impl From<kernel::error::KernelError> for ApplicationError {
    fn from(value: kernel::error::KernelError) -> Self {
        Self::Kernel(value)
    }
}