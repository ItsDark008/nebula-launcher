use serde::Serialize;

pub mod accounts;
pub mod auth;
pub mod minecraft_auth;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Registration {
    Registered,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Review {
    Submitted,
    Approved,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProtectedAccess {
    Verified,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftAccessStatus {
    pub registration: Registration,
    pub review_submission: Review,
    pub review_approval: Review,
    pub protected_access: ProtectedAccess,
}

pub fn minecraft_access_status() -> MinecraftAccessStatus {
    MinecraftAccessStatus {
        registration: Registration::Registered,
        review_submission: Review::Submitted,
        review_approval: Review::Approved,
        protected_access: ProtectedAccess::Verified,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn initial_status_records_each_access_milestone() {
        assert_eq!(
            serde_json::to_value(minecraft_access_status()).unwrap(),
            json!({
                "registration": "registered",
                "reviewSubmission": "submitted",
                "reviewApproval": "approved",
                "protectedAccess": "verified"
            })
        );
    }
}
