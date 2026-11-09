//! Category entity. V3: slug non-empty, lowercase.

use serde::{Deserialize, Serialize};

use super::DomainError;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Category {
    pub id: String,
    pub slug: String,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewCategory {
    pub slug: String,
    pub name: String,
}

impl NewCategory {
    /// V3: ∀ Category → slug ∉ ∅, slug = to_lowercase(slug)
    pub fn validate(&self) -> Result<(), DomainError> {
        if self.slug.trim().is_empty() {
            return Err(DomainError::EmptySlug);
        }
        if self.slug != self.slug.to_lowercase() {
            return Err(DomainError::SlugNotLowercase(self.slug.clone()));
        }
        if self.name.trim().is_empty() {
            return Err(DomainError::EmptyName);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_slug() {
        let c = NewCategory {
            slug: "".into(),
            name: "Bebidas".into(),
        };
        assert!(matches!(c.validate(), Err(DomainError::EmptySlug)));
    }

    #[test]
    fn rejects_uppercase_slug() {
        let c = NewCategory {
            slug: "Bebidas".into(),
            name: "Bebidas".into(),
        };
        assert!(matches!(
            c.validate(),
            Err(DomainError::SlugNotLowercase(_))
        ));
    }

    #[test]
    fn accepts_lowercase_slug() {
        let c = NewCategory {
            slug: "bebidas".into(),
            name: "Bebidas".into(),
        };
        c.validate().unwrap();
    }
}
