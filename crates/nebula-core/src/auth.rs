//! Public-client authorization setup. Secrets remain in the Rust process.
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use sha2::{Digest, Sha256};
use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
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
    ListenerUnavailable,
    TokenExchangeFailed,
}

/// Provider credentials remain in Rust and deliberately do not implement
/// Debug or Serialize. They must be stored in the system keyring, never IPC.
#[derive(serde::Deserialize)]
pub struct MicrosoftTokens {
    pub(crate) access_token: String,
    pub(crate) refresh_token: String,
}

/// Redeem a successful callback code using the same redirect and verifier.
/// The response and provider error bodies are never returned to the UI or logs.
pub fn redeem_code(
    attempt: &AuthorizationAttempt,
    code: &str,
) -> Result<MicrosoftTokens, AuthorizationError> {
    if !attempt.used || code.is_empty() {
        return Err(AuthorizationError::TokenExchangeFailed);
    }
    let client = reqwest::blocking::Client::builder()
        .https_only(true)
        .redirect(reqwest::redirect::Policy::none())
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|_| AuthorizationError::TokenExchangeFailed)?;
    let mut response = client
        .post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
        .form(&[
            ("client_id", attempt.client_id.as_str()),
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", attempt.redirect_uri().as_str()),
            ("code_verifier", attempt.verifier()),
            ("scope", "XboxLive.signin offline_access"),
        ])
        .send()
        .map_err(|_| AuthorizationError::TokenExchangeFailed)?;
    if !response.status().is_success() {
        return Err(AuthorizationError::TokenExchangeFailed);
    }
    let mut body = Vec::new();
    response
        .by_ref()
        .take(65537)
        .read_to_end(&mut body)
        .map_err(|_| AuthorizationError::TokenExchangeFailed)?;
    if body.len() > 65536 {
        return Err(AuthorizationError::TokenExchangeFailed);
    }
    parse_microsoft_tokens(&body)
}

fn parse_microsoft_tokens(body: &[u8]) -> Result<MicrosoftTokens, AuthorizationError> {
    let tokens: MicrosoftTokens =
        serde_json::from_slice(body).map_err(|_| AuthorizationError::TokenExchangeFailed)?;
    if tokens.access_token.is_empty() || tokens.refresh_token.is_empty() {
        return Err(AuthorizationError::TokenExchangeFailed);
    }
    Ok(tokens)
}

/// A loopback listener owns a single attempt and never exposes its credentials
/// to a caller until a correlated authorization response has arrived.
pub struct LoopbackAuthorization {
    listener: TcpListener,
    attempt: AuthorizationAttempt,
}

impl LoopbackAuthorization {
    pub fn bind(client_id: &str) -> Result<(Self, Url), AuthorizationError> {
        let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0))
            .map_err(|_| AuthorizationError::ListenerUnavailable)?;
        listener
            .set_nonblocking(true)
            .map_err(|_| AuthorizationError::ListenerUnavailable)?;
        let port = listener
            .local_addr()
            .map_err(|_| AuthorizationError::ListenerUnavailable)?
            .port();
        let (attempt, url) = AuthorizationAttempt::new(client_id, port)?;
        Ok((Self { listener, attempt }, url))
    }

    /// Wait at most until the attempt expires. A malformed request receives a
    /// fixed error and cannot consume the valid attempt.
    pub fn wait(&mut self) -> Result<String, AuthorizationError> {
        loop {
            if Instant::now() >= self.attempt.expires_at {
                return Err(AuthorizationError::Expired);
            }
            match self.listener.accept() {
                Ok((mut stream, peer)) if peer.ip().is_loopback() => {
                    let result = self.read_callback(&mut stream);
                    let (status, body) = if result.is_ok() {
                        (
                            "200 OK",
                            "Authorization received. You may close this window.",
                        )
                    } else {
                        (
                            "400 Bad Request",
                            "Unable to process the authorization response.",
                        )
                    };
                    let _ = write!(
                        stream,
                        "HTTP/1.1 {status}\r\nContent-Type: text/plain; charset=utf-8\r\nCache-Control: no-store\r\nConnection: close\r\nContent-Length: {}\r\n\r\n{body}",
                        body.len()
                    );
                    if result.is_ok() {
                        return result;
                    }
                }
                Ok(_) => (),
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    std::thread::sleep(Duration::from_millis(20));
                }
                Err(_) => return Err(AuthorizationError::ListenerUnavailable),
            }
        }
    }

    fn read_callback(&mut self, stream: &mut TcpStream) -> Result<String, AuthorizationError> {
        let remaining = self
            .attempt
            .expires_at
            .saturating_duration_since(Instant::now());
        stream
            .set_read_timeout(Some(remaining.min(Duration::from_secs(2))))
            .map_err(|_| AuthorizationError::InvalidCallback)?;
        let mut data = Vec::with_capacity(1024);
        let mut chunk = [0u8; 1024];
        while !data.ends_with(b"\r\n\r\n") && data.len() < 8192 {
            let n = stream
                .read(&mut chunk)
                .map_err(|_| AuthorizationError::InvalidCallback)?;
            if n == 0 {
                return Err(AuthorizationError::InvalidCallback);
            }
            data.extend_from_slice(&chunk[..n]);
        }
        if data.len() > 8192 || !data.ends_with(b"\r\n\r\n") {
            return Err(AuthorizationError::InvalidCallback);
        }
        let text = std::str::from_utf8(&data).map_err(|_| AuthorizationError::InvalidCallback)?;
        let mut lines = text.split("\r\n");
        let mut request = lines
            .next()
            .ok_or(AuthorizationError::InvalidCallback)?
            .split(' ');
        if request.next() != Some("GET") {
            return Err(AuthorizationError::InvalidCallback);
        }
        let target = request.next().ok_or(AuthorizationError::InvalidCallback)?;
        if !target.starts_with("/?")
            || request.next() != Some("HTTP/1.1")
            || request.next().is_some()
        {
            return Err(AuthorizationError::InvalidCallback);
        }
        let expected_host = format!(
            "localhost:{}",
            self.attempt.redirect_uri().port().unwrap_or(80)
        );
        let hosts: Vec<_> = lines
            .filter_map(|line| line.split_once(':'))
            .filter(|(key, _)| key.eq_ignore_ascii_case("host"))
            .collect();
        if hosts.len() != 1 || !hosts[0].1.trim().eq_ignore_ascii_case(&expected_host) {
            return Err(AuthorizationError::InvalidCallback);
        }
        let url = Url::parse(&format!("http://{expected_host}{target}"))
            .map_err(|_| AuthorizationError::InvalidCallback)?;
        self.attempt.accept(&url, Instant::now())
    }

    pub fn attempt(&self) -> &AuthorizationAttempt {
        &self.attempt
    }
}

/// One browser authorization attempt. Do not derive Debug: the verifier and
/// returned authorization code must never be written to logs or the WebView.
pub struct AuthorizationAttempt {
    client_id: String,
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
                client_id: client_id.to_owned(),
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
    fn token_response_requires_both_nonempty_credentials() {
        assert!(
            parse_microsoft_tokens(br#"{"access_token":"fixture","refresh_token":"fixture"}"#)
                .is_ok()
        );
        assert!(parse_microsoft_tokens(br#"{"access_token":"fixture"}"#).is_err());
        assert!(
            parse_microsoft_tokens(br#"{"access_token":"fixture","refresh_token":""}"#).is_err()
        );
        assert!(parse_microsoft_tokens(b"not-json").is_err());
    }

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

    #[test]
    fn listener_rejects_wrong_host_and_accepts_correlated_response() {
        let (mut flow, authorization_url) = LoopbackAuthorization::bind(CLIENT).unwrap();
        let port = flow.attempt.redirect_uri().port().unwrap();
        let state = authorization_url
            .query_pairs()
            .find(|(key, _)| key == "state")
            .unwrap()
            .1
            .into_owned();
        let worker = std::thread::spawn(move || flow.wait());
        let send = |host: &str| {
            let mut stream = TcpStream::connect((Ipv4Addr::LOCALHOST, port)).unwrap();
            write!(
                stream,
                "GET /?code=fixture&state={state} HTTP/1.1\r\nHost: {host}\r\n\r\n"
            )
            .unwrap();
            let mut response = String::new();
            stream.read_to_string(&mut response).unwrap();
            response
        };
        assert!(send("evil.example").starts_with("HTTP/1.1 400"));
        assert!(send(&format!("localhost:{port}")).starts_with("HTTP/1.1 200"));
        assert_eq!(worker.join().unwrap().unwrap(), "fixture");
    }
}
