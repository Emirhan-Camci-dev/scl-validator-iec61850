use crate::types::SclDocument;

pub struct SclValidator;

#[derive(Debug)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
}

impl Default for SclValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl SclValidator {
    pub fn new() -> Self {
        Self
    }

    pub fn validate(&self, doc: &SclDocument) -> ValidationResult {
        let mut errors = Vec::new();

        if doc.ieds.is_empty() {
            errors.push("SCL document contains no IEDs.".to_string());
        }

        for ied in &doc.ieds {
            if ied.name.is_empty() {
                errors.push("IED found with empty name.".to_string());
            }
        }

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
        }
    }
}
