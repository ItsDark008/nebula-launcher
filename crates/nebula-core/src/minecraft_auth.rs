//! Minecraft service exchange. Provider credentials stay inside the core.
use crate::auth::MicrosoftTokens;
use reqwest::blocking::{Client, Response};
use serde::Serialize;
use serde_json::{Value, json};
use std::io::Read;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MinecraftAuthError {
    Network,
    Xbox,
    Xsts,
    Minecraft,
    Entitlement,
    Profile,
}

/// Only this identity is suitable for IPC; no provider token is included.
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct MinecraftProfile {
    pub id: String,
    pub name: String,
}

/// Not serializable or printable. Access tokens must remain in Rust memory.
pub struct MinecraftSession {
    pub profile: MinecraftProfile,
    #[allow(dead_code)] // Consumed by the authenticated launch phase.
    pub(crate) access_token: String,
}

fn read_json(
    mut response: Response,
    error: MinecraftAuthError,
) -> Result<Value, MinecraftAuthError> {
    if !response.status().is_success() {
        return Err(error);
    }
    let mut bytes = Vec::new();
    response
        .by_ref()
        .take(65537)
        .read_to_end(&mut bytes)
        .map_err(|_| error)?;
    if bytes.len() > 65536 {
        return Err(error);
    }
    serde_json::from_slice(&bytes).map_err(|_| error)
}

fn nonempty<'a>(
    value: &'a Value,
    pointer: &str,
    error: MinecraftAuthError,
) -> Result<&'a str, MinecraftAuthError> {
    value
        .pointer(pointer)
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty())
        .ok_or(error)
}

trait Transport {
    fn post(
        &self,
        url: &str,
        body: &Value,
        stage: MinecraftAuthError,
    ) -> Result<Value, MinecraftAuthError>;
    fn get(
        &self,
        url: &str,
        token: &str,
        stage: MinecraftAuthError,
    ) -> Result<Value, MinecraftAuthError>;
}

impl Transport for Client {
    fn post(
        &self,
        url: &str,
        body: &Value,
        stage: MinecraftAuthError,
    ) -> Result<Value, MinecraftAuthError> {
        read_json(self.post(url).json(body).send().map_err(|_| stage)?, stage)
    }
    fn get(
        &self,
        url: &str,
        token: &str,
        stage: MinecraftAuthError,
    ) -> Result<Value, MinecraftAuthError> {
        read_json(
            self.get(url).bearer_auth(token).send().map_err(|_| stage)?,
            stage,
        )
    }
}

/// Perform provider exchanges and require a Java entitlement before returning a profile.
pub fn authenticate(tokens: &MicrosoftTokens) -> Result<MinecraftSession, MinecraftAuthError> {
    let client = Client::builder()
        .https_only(true)
        .redirect(reqwest::redirect::Policy::none())
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|_| MinecraftAuthError::Network)?;
    authenticate_with(&client, tokens)
}

fn authenticate_with(
    client: &impl Transport,
    tokens: &MicrosoftTokens,
) -> Result<MinecraftSession, MinecraftAuthError> {
    let xbox = client.post("https://user.auth.xboxlive.com/user/authenticate", &json!({
        "Properties": { "AuthMethod": "RPS", "SiteName": "user.auth.xboxlive.com", "RpsTicket": format!("d={}", tokens.access_token) },
        "RelyingParty": "http://auth.xboxlive.com", "TokenType": "JWT"
    }), MinecraftAuthError::Xbox)?;
    let xbox_token = nonempty(&xbox, "/Token", MinecraftAuthError::Xbox)?;
    let xsts = client.post(
        "https://xsts.auth.xboxlive.com/xsts/authorize",
        &json!({
            "Properties": { "SandboxId": "RETAIL", "UserTokens": [xbox_token] },
            "RelyingParty": "rp://api.minecraftservices.com/", "TokenType": "JWT"
        }),
        MinecraftAuthError::Xsts,
    )?;
    let xsts_token = nonempty(&xsts, "/Token", MinecraftAuthError::Xsts)?;
    let user_hash = nonempty(&xsts, "/DisplayClaims/xui/0/uhs", MinecraftAuthError::Xsts)?;
    let minecraft = client.post(
        "https://api.minecraftservices.com/authentication/login_with_xbox",
        &json!({"identityToken": format!("XBL3.0 x={user_hash};{xsts_token}")}),
        MinecraftAuthError::Minecraft,
    )?;
    let access_token =
        nonempty(&minecraft, "/access_token", MinecraftAuthError::Minecraft)?.to_owned();
    let entitlements = client.get(
        "https://api.minecraftservices.com/entitlements/mcstore",
        &access_token,
        MinecraftAuthError::Entitlement,
    )?;
    if !entitlements
        .get("items")
        .and_then(Value::as_array)
        .is_some_and(|items| {
            items
                .iter()
                .any(|item| item.get("name").and_then(Value::as_str) == Some("game_minecraft"))
        })
    {
        return Err(MinecraftAuthError::Entitlement);
    }
    let profile = client.get(
        "https://api.minecraftservices.com/minecraft/profile",
        &access_token,
        MinecraftAuthError::Profile,
    )?;
    let id = nonempty(&profile, "/id", MinecraftAuthError::Profile)?.to_owned();
    let name = nonempty(&profile, "/name", MinecraftAuthError::Profile)?.to_owned();
    Ok(MinecraftSession {
        profile: MinecraftProfile { id, name },
        access_token,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::VecDeque;

    struct Fixture(RefCell<VecDeque<(MinecraftAuthError, Value)>>);
    impl Transport for Fixture {
        fn post(
            &self,
            _: &str,
            _: &Value,
            stage: MinecraftAuthError,
        ) -> Result<Value, MinecraftAuthError> {
            self.next(stage)
        }
        fn get(
            &self,
            _: &str,
            _: &str,
            stage: MinecraftAuthError,
        ) -> Result<Value, MinecraftAuthError> {
            self.next(stage)
        }
    }
    impl Fixture {
        fn next(&self, stage: MinecraftAuthError) -> Result<Value, MinecraftAuthError> {
            let (expected, value) = self.0.borrow_mut().pop_front().expect("unexpected request");
            assert_eq!(stage, expected);
            Ok(value)
        }
    }
    fn tokens() -> MicrosoftTokens {
        serde_json::from_value(json!({"access_token": "fixture", "refresh_token": "fixture"}))
            .unwrap()
    }
    fn fixture(entitlements: Value) -> Fixture {
        Fixture(RefCell::new(VecDeque::from([
            (MinecraftAuthError::Xbox, json!({"Token": "fixture-xbox"})),
            (
                MinecraftAuthError::Xsts,
                json!({"Token": "fixture-xsts", "DisplayClaims": {"xui": [{"uhs": "fixture"}]}}),
            ),
            (
                MinecraftAuthError::Minecraft,
                json!({"access_token": "fixture-minecraft"}),
            ),
            (MinecraftAuthError::Entitlement, entitlements),
            (
                MinecraftAuthError::Profile,
                json!({"id": "fixture-id", "name": "Player"}),
            ),
        ])))
    }
    #[test]
    fn requires_entitlement_before_returning_profile() {
        let provider = fixture(json!({"items": [{"name": "game_minecraft"}]}));
        let result = authenticate_with(&provider, &tokens()).unwrap();
        assert_eq!(result.profile.name, "Player");
        assert!(provider.0.borrow().is_empty());
        let provider = fixture(json!({"items": []}));
        assert!(matches!(
            authenticate_with(&provider, &tokens()),
            Err(MinecraftAuthError::Entitlement)
        ));
        assert_eq!(provider.0.borrow().len(), 1);
    }
    #[test]
    fn rejects_invalid_xbox_token_before_xsts() {
        let provider = Fixture(RefCell::new(VecDeque::from([(
            MinecraftAuthError::Xbox,
            json!({"Token": ""}),
        )])));
        assert!(matches!(
            authenticate_with(&provider, &tokens()),
            Err(MinecraftAuthError::Xbox)
        ));
    }
}
