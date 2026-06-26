//! Supplier entity. NIT (Colombian tax ID) is optional but unique when set.

use serde::{Deserialize, Serialize};

use super::DomainError;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Supplier {
    pub id: String,
    pub name: String,
    pub nit: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewSupplier {
    pub name: String,
    pub nit: Option<String>,
}

impl NewSupplier {
    pub fn validate(&self) -> Result<(), DomainError> {
        if self.name.trim().is_empty() {
            return Err(DomainError::EmptyName);
        }
        Ok(())
    }
}
