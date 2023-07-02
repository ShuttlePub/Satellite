use kernel::prelude::entities::Identifier;

#[derive(Debug)]
pub struct IdentifierDto { pub id: String }

impl From<Identifier> for IdentifierDto {
    fn from(value: Identifier) -> Self {
        Self { id: value.into() }
    }
}