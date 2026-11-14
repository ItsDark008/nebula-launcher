//! Public-client authorization setup. Secrets remain in the Rust process.
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use sha2::{Digest, Sha256};
use std::time::{Duration, Instant};
use url::Url;

const AUTHORIZATION_ENDPOINT: &str =
    "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize";
const LIFETIME: Duration = Duration::from_secs(300);

#[derive(Debug, PartialEq, Eq)]
pub enum AuthorizationError {
    RandomUnavailable,
    InvalidClient,
    InvalidCallback,
    Expired,
    AlreadyUsed,
}

/// One browser authorization attempt. Do not derive Debug: the verifier and
/// returned authorization code must never be written to logs or the WebView.
pub struct AuthorizationAttempt {
    state: String,
    verifier: String,
    callback: Url,
    expires_at: Instant,
    used: bool,
}

impl AuthorizationAttempt {
    pub fn new(client_id: &str, port: u16) -> Result<(Self, Url), AuthorizationError> {
        if port == 0
            || client_id.len() != 36
            || !client_id.bytes().enumerate().all(|(i, c)| {
                if [8, 13, 18, 23].contains(&i) {
                    c == b'-'
                } else {
                    c.is_ascii_hexdigit()
                }
            })
        {
            return Err(AuthorizationError::InvalidClient);
        }
        let mut bytes = [0u8; 64];
        getrandom::fill(&mut bytes).map_err(|_| AuthorizationError::RandomUnavailable)?;
        let state = URL_SAFE_NO_PAD.encode(&bytes[..32]);
        let verifier = URL_SAFE_NO_PAD.encode(&bytes[32..]);
        let challenge = URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes()));
        let callback = Url::parse(&format!("http://localhost:{port}/"))
            .map_err(|_| AuthorizationError::InvalidCallback)?;
        let mut authorization_url =
            Url::parse(AUTHORIZATION_ENDPOINT).map_err(|_| AuthorizationError::InvalidCallback)?;
        authorization_url
            .query_pairs_mut()
            .append_pair("client_id", client_id)
            .append_pair("response_type", "code")
            .append_pair("redirect_uri", callback.as_str())
            .append_pair("response_mode", "query")
            .append_pair("scope", "XboxLive.signin offline_access")
            .append_pair("state", &state)
            .append_pair("code_challenge", &challenge)
            .append_pair("code_challenge_method", "S256");
        Ok((
            Self {
                state,
                verifier,
                callback,
                expires_at: Instant::now() + LIFETIME,
                used: false,
            },
            authorization_url,
        ))
    }

    /// Consume a correlated response exactly once. Callers must bound the
    /// loopback listener, request size, and wait time independently.
    pub fn accept(&mut self, response: &Url, now: Instant) -> Result<String, AuthorizationError> {
        if self.used {
            return Err(AuthorizationError::AlreadyUsed);
        }
        if now >= self.expires_at {
            return Err(AuthorizationError::Expired);
        }
        if response.scheme() != self.callback.scheme()
            || response.host_str() != self.callback.host_str()
            || response.port_or_known_default() != self.callback.port_or_known_default()
            || response.path() != self.callback.path()
            || !response.username().is_empty()
            || response.password().is_some()
            || response.fragment().is_some()
        {
            return Err(AuthorizationError::InvalidCallback);
        }
        let mut state = None;
        let mut code = None;
        for (key, value) in response.query_pairs() {
            match key.as_ref() {
                "state" if state.is_none() => state = Some(value.into_owned()),
                "code" if code.is_none() => code = Some(value.into_owned()),
                "state" | "code" | "error" => return Err(AuthorizationError::InvalidCallback),
                _ => (),
            }
        }
        let state = state.ok_or(AuthorizationError::InvalidCallback)?;
        let code = code
            .filter(|s| !s.is_empty())
            .ok_or(AuthorizationError::InvalidCallback)?;
        if state.len() != self.state.len()
            || !state
                .bytes()
                .zip(self.state.bytes())
                .fold(0u8, |acc, (a, b)| acc | (a ^ b))
                .eq(&0)
        {
            return Err(AuthorizationError::InvalidCallback);
        }
        self.used = true;
        Ok(code)
    }

    pub fn verifier(&self) -> &str {
        &self.verifier
    }
    pub fn redirect_uri(&self) -> &Url {
        &self.callback
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const CLIENT: &str = "00000000-0000-4000-8000-000000000000";

    #[test]
    fn builds_pkce_and_consumes_correlated_callback_once() {
        let (mut attempt, url) = AuthorizationAttempt::new(CLIENT, 45678).unwrap();
        let pairs: std::collections::HashMap<_, _> = url.query_pairs().into_owned().collect();
        assert_eq!(pairs["code_challenge_method"], "S256");
        assert_eq!(
            pairs["code_challenge"],
            URL_SAFE_NO_PAD.encode(Sha256::digest(attempt.verifier().as_bytes()))
        );
        let mut response = attempt.redirect_uri().clone();
        response
            .query_pairs_mut()
            .append_pair("state", &pairs["state"])
            .append_pair("code", "fixture-code");
        assert_eq!(
            attempt.accept(&response, Instant::now()).unwrap(),
            "fixture-code"
        );
        assert_eq!(
            attempt.accept(&response, Instant::now()),
            Err(AuthorizationError::AlreadyUsed)
        );
    }

    #[test]
    fn rejects_wrong_state_duplicate_parameters_expiration_and_wrong_origin() {
        let (mut attempt, url) = AuthorizationAttempt::new(CLIENT, 45678).unwrap();
        let state = url
            .query_pairs()
            .find(|(key, _)| key == "state")
            .unwrap()
            .1
            .into_owned();
        let mut response = attempt.redirect_uri().clone();
        response
            .query_pairs_mut()
            .append_pair("state", "wrong")
            .append_pair("code", "fixture");
        assert_eq!(
            attempt.accept(&response, Instant::now()),
            Err(AuthorizationError::InvalidCallback)
        );
        response.set_query(None);
        response
            .query_pairs_mut()
            .append_pair("state", &state)
            .append_pair("state", &state)
            .append_pair("code", "fixture");
        assert_eq!(
            attempt.accept(&response, Instant::now()),
            Err(AuthorizationError::InvalidCallback)
        );
        response.set_query(None);
        response
            .query_pairs_mut()
            .append_pair("state", &state)
            .append_pair("code", "fixture");
        response.set_port(Some(45679)).unwrap();
        assert_eq!(
            attempt.accept(&response, Instant::now()),
            Err(AuthorizationError::InvalidCallback)
        );
        assert_eq!(
            attempt.accept(&response, attempt.expires_at),
            Err(AuthorizationError::Expired)
        );
    }
}
