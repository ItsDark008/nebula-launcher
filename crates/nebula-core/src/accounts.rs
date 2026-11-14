//! Secret Service backed account metadata and refresh credentials.
use crate::minecraft_auth::MinecraftProfile;
use serde::{Deserialize, Serialize};
use std::sync::{Mutex, OnceLock};

const SERVICE: &str = "nebula-launcher";
const KEY: &str = "accounts-v1";
static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
static SESSIONS: OnceLock<
    Mutex<std::collections::HashMap<String, crate::minecraft_auth::MinecraftSession>>,
> = OnceLock::new();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreError {
    Unavailable,
    LockedOrDenied,
    InvalidData,
    RefreshFailed,
    ReauthenticationRequired,
}

fn store_error(error: keyring::Error) -> StoreError {
    match error {
        keyring::Error::NoStorageAccess(_) => StoreError::LockedOrDenied,
        _ => StoreError::Unavailable,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignInError {
    Keyring,
    Browser,
    Microsoft,
    Xbox,
    Minecraft,
}

pub fn sign_in(
    client_id: &str,
    open_browser: impl FnOnce(&url::Url) -> Result<(), ()>,
) -> Result<MinecraftProfile, SignInError> {
    // Ensure the secure store is available before requesting credentials.
    list().map_err(|_| SignInError::Keyring)?;
    let (mut flow, url) =
        crate::auth::LoopbackAuthorization::bind(client_id).map_err(|_| SignInError::Microsoft)?;
    open_browser(&url).map_err(|_| SignInError::Browser)?;
    let code = flow.wait().map_err(|_| SignInError::Microsoft)?;
    let tokens =
        crate::auth::redeem_code(flow.attempt(), &code).map_err(|_| SignInError::Microsoft)?;
    let session = crate::minecraft_auth::authenticate(&tokens).map_err(|error| match error {
        crate::minecraft_auth::MinecraftAuthError::Xbox
        | crate::minecraft_auth::MinecraftAuthError::Xsts => SignInError::Xbox,
        _ => SignInError::Minecraft,
    })?;
    put(&session.profile, &tokens.refresh_token).map_err(|_| SignInError::Keyring)?;
    let profile = session.profile.clone();
    SESSIONS
        .get_or_init(|| Mutex::new(std::collections::HashMap::new()))
        .lock()
        .map_err(|_| SignInError::Keyring)?
        .insert(profile.id.clone(), session);
    Ok(profile)
}

pub fn refresh_active(client_id: &str) -> Result<MinecraftProfile, StoreError> {
    locked(|| {
        let mut data = load()?;
        let id = data.active.clone().ok_or(StoreError::InvalidData)?;
        let account = data
            .accounts
            .iter_mut()
            .find(|a| a.id == id)
            .ok_or(StoreError::InvalidData)?;
        let tokens =
            crate::auth::refresh_tokens(client_id, &account.refresh_token).map_err(|error| {
                match error {
                    crate::auth::AuthorizationError::ReauthenticationRequired => {
                        StoreError::ReauthenticationRequired
                    }
                    _ => StoreError::RefreshFailed,
                }
            })?;
        account.refresh_token = tokens.refresh_token.clone();
        save(&data)?;
        let session =
            crate::minecraft_auth::authenticate(&tokens).map_err(|_| StoreError::RefreshFailed)?;
        if session.profile.id != id {
            return Err(StoreError::InvalidData);
        }
        let profile = session.profile.clone();
        SESSIONS
            .get_or_init(|| Mutex::new(std::collections::HashMap::new()))
            .lock()
            .map_err(|_| StoreError::Unavailable)?
            .insert(id, session);
        Ok(profile)
    })
}

#[derive(Serialize, Deserialize, Default)]
struct StoredAccounts {
    active: Option<String>,
    accounts: Vec<StoredAccount>,
}

#[derive(Serialize, Deserialize)]
struct StoredAccount {
    id: String,
    name: String,
    refresh_token: String,
}

impl StoredAccounts {
    fn into_public(self) -> AccountList {
        AccountList {
            active: self.active,
            accounts: self
                .accounts
                .into_iter()
                .map(|account| MinecraftProfile {
                    id: account.id,
                    name: account.name,
                })
                .collect(),
        }
    }
    fn put(&mut self, profile: &MinecraftProfile, refresh_token: &str) {
        self.accounts.retain(|account| account.id != profile.id);
        self.accounts.push(StoredAccount {
            id: profile.id.clone(),
            name: profile.name.clone(),
            refresh_token: refresh_token.to_owned(),
        });
        self.active = Some(profile.id.clone());
    }
    fn select(&mut self, id: &str) -> Result<(), StoreError> {
        if !self.accounts.iter().any(|a| a.id == id) {
            return Err(StoreError::InvalidData);
        }
        self.active = Some(id.to_owned());
        Ok(())
    }
    fn remove(&mut self, id: &str) {
        self.accounts.retain(|a| a.id != id);
        if self.active.as_deref() == Some(id) {
            self.active = self.accounts.first().map(|a| a.id.clone());
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountList {
    pub active: Option<String>,
    pub accounts: Vec<MinecraftProfile>,
}

fn entry() -> Result<keyring::Entry, StoreError> {
    keyring::Entry::new(SERVICE, KEY).map_err(store_error)
}
fn load() -> Result<StoredAccounts, StoreError> {
    match entry()?.get_password() {
        Ok(data) => serde_json::from_str(&data).map_err(|_| StoreError::InvalidData),
        Err(keyring::Error::NoEntry) => Ok(StoredAccounts::default()),
        Err(error) => Err(store_error(error)),
    }
}
fn save(accounts: &StoredAccounts) -> Result<(), StoreError> {
    let data = serde_json::to_string(accounts).map_err(|_| StoreError::InvalidData)?;
    entry()?.set_password(&data).map_err(store_error)
}
fn locked<T>(f: impl FnOnce() -> Result<T, StoreError>) -> Result<T, StoreError> {
    let _guard = LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .map_err(|_| StoreError::Unavailable)?;
    f()
}
pub fn list() -> Result<AccountList, StoreError> {
    locked(|| Ok(load()?.into_public()))
}
pub fn put(profile: &MinecraftProfile, refresh_token: &str) -> Result<(), StoreError> {
    if refresh_token.is_empty() || profile.id.is_empty() {
        return Err(StoreError::InvalidData);
    }
    locked(|| {
        let mut data = load()?;
        data.put(profile, refresh_token);
        save(&data)
    })
}
pub fn select(id: &str) -> Result<(), StoreError> {
    locked(|| {
        let mut data = load()?;
        data.select(id)?;
        save(&data)
    })
}
pub fn remove(id: &str) -> Result<(), StoreError> {
    locked(|| {
        let mut data = load()?;
        data.remove(id);
        SESSIONS
            .get_or_init(|| Mutex::new(std::collections::HashMap::new()))
            .lock()
            .map_err(|_| StoreError::Unavailable)?
            .remove(id);
        if data.accounts.is_empty() {
            match entry()?.delete_credential() {
                Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
                Err(error) => Err(store_error(error)),
            }
        } else {
            save(&data)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn switches_and_removes_accounts_without_affecting_others() {
        let mut state = StoredAccounts::default();
        let a = MinecraftProfile {
            id: "a".into(),
            name: "Player A".into(),
        };
        let b = MinecraftProfile {
            id: "b".into(),
            name: "Player B".into(),
        };
        state.put(&a, "fixture-a");
        state.put(&b, "fixture-b");
        state.select("a").unwrap();
        assert_eq!(state.active.as_deref(), Some("a"));
        state.remove("a");
        assert_eq!(state.active.as_deref(), Some("b"));
        assert_eq!(state.accounts.len(), 1);
        assert_eq!(state.accounts[0].refresh_token, "fixture-b");
        assert_eq!(state.select("a"), Err(StoreError::InvalidData));
    }

    #[test]
    fn public_account_list_never_serializes_refresh_credentials() {
        let mut state = StoredAccounts::default();
        state.put(
            &MinecraftProfile {
                id: "fixture-id".into(),
                name: "Player".into(),
            },
            "fixture-secret",
        );
        let json = serde_json::to_string(&state.into_public()).unwrap();
        assert!(!json.contains("fixture-secret"));
        assert!(!json.contains("refresh_token"));
        assert!(json.contains("Player"));
    }
}
