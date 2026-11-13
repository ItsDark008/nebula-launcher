use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Registration {
    AwaitingOwnerRegistration,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Review {
    Pending,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProtectedAccess {
    Blocked,
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
        registration: Registration::AwaitingOwnerRegistration,
        review_submission: Review::Pending,
        review_approval: Review::Pending,
        protected_access: ProtectedAccess::Blocked,
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
                "registration": "awaiting_owner_registration",
                "reviewSubmission": "pending",
                "reviewApproval": "pending",
                "protectedAccess": "blocked"
            })
        );
    }
}
