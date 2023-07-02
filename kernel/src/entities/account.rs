mod user_id;
mod address;
mod name;
mod pass;

pub use self::{
    user_id::*,
    address::*,
    name::*,
    pass::*,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Hash, Deserialize, Serialize)]
pub struct Account {
    id: UserId,
    address: Address,
    name: UserName,
}