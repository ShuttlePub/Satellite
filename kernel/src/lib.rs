mod entities;
mod repository;
mod transport;

pub mod error;

#[cfg(feature = "prelude")]
pub mod prelude {
    pub mod entities { pub use crate::entities::*; }
}

#[cfg(feature = "interfaces")]
pub mod interfaces {
    pub mod repository { pub use crate::repository::*; }
    pub mod transport  { pub use crate::transport::*;  }
}