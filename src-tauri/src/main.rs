use nebula_core::{MinecraftAccessStatus, minecraft_access_status};
use serde::Serialize;
use std::os::unix::fs::OpenOptionsExt;
use std::sync::Mutex;
use tauri::{Emitter, EventTarget, Runtime, WebviewWindow};

static ACCOUNT_OPERATION: Mutex<()> = Mutex::new(());

fn account_guard() -> Result<(std::sync::MutexGuard<'static, ()>, std::fs::File), &'static str> {
    let guard = ACCOUNT_OPERATION
        .lock()
        .map_err(|_| "Accounts are unavailable.")?;
    let runtime = std::env::var_os("XDG_RUNTIME_DIR").ok_or("Accounts are unavailable.")?;
    let path = std::path::PathBuf::from(runtime).join("nebula-launcher-accounts.lock");
    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .mode(0o600)
        .open(path)
        .map_err(|_| "Accounts are unavailable.")?;
    fs2::FileExt::lock_exclusive(&file).map_err(|_| "Accounts are unavailable.")?;
    Ok((guard, file))
}

fn client_id() -> Result<String, &'static str> {
    std::env::var("NEBULA_MICROSOFT_CLIENT_ID").map_err(|_| "Microsoft sign-in is not configured.")
}

fn store_error(error: nebula_core::accounts::StoreError) -> &'static str {
    match error {
        nebula_core::accounts::StoreError::LockedOrDenied => {
            "The system keyring is locked or access was denied."
        }
        nebula_core::accounts::StoreError::Unavailable => "The system keyring is unavailable.",
        nebula_core::accounts::StoreError::InvalidData => "The stored account is unavailable.",
        nebula_core::accounts::StoreError::RefreshFailed => {
            "Unable to restore this session. Check your connection and retry."
        }
        nebula_core::accounts::StoreError::ReauthenticationRequired => {
            "The session expired or was revoked. Sign in again."
        }
    }
}

#[tauri::command]
async fn list_accounts() -> Result<nebula_core::accounts::AccountList, &'static str> {
    tauri::async_runtime::spawn_blocking(|| {
        let _guard = account_guard()?;
        nebula_core::accounts::list().map_err(store_error)
    })
    .await
    .map_err(|_| "Accounts are unavailable.")?
}

#[tauri::command]
async fn sign_in_account() -> Result<nebula_core::minecraft_auth::MinecraftProfile, &'static str> {
    tauri::async_runtime::spawn_blocking(|| {
        let _guard = account_guard()?;
        nebula_core::accounts::sign_in(&client_id()?, |url| {
            std::process::Command::new("xdg-open")
                .arg(url.as_str())
                .stdin(std::process::Stdio::null())
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn()
                .map(|_| ())
                .map_err(|_| ())
        })
        .map_err(|error| match error {
            nebula_core::accounts::SignInError::Keyring => {
                "The system keyring is unavailable or locked."
            }
            nebula_core::accounts::SignInError::Browser => "Could not open the system browser.",
            nebula_core::accounts::SignInError::Microsoft => {
                "Microsoft sign-in failed or expired. Try again."
            }
            nebula_core::accounts::SignInError::Xbox => {
                "Xbox sign-in failed. Check your Xbox account and try again."
            }
            nebula_core::accounts::SignInError::Minecraft => {
                "Minecraft access or ownership could not be verified."
            }
        })
    })
    .await
    .map_err(|_| "Sign-in is unavailable.")?
}

#[tauri::command]
async fn select_account(id: String) -> Result<(), &'static str> {
    tauri::async_runtime::spawn_blocking(move || {
        let _guard = account_guard()?;
        nebula_core::accounts::select(&id).map_err(|_| "Unable to select this account.")
    })
    .await
    .map_err(|_| "Accounts are unavailable.")?
}

#[tauri::command]
async fn remove_account(id: String) -> Result<(), &'static str> {
    tauri::async_runtime::spawn_blocking(move || {
        let _guard = account_guard()?;
        nebula_core::accounts::remove(&id).map_err(|_| "Unable to remove this account.")
    })
    .await
    .map_err(|_| "Accounts are unavailable.")?
}

#[tauri::command]
async fn refresh_account() -> Result<nebula_core::minecraft_auth::MinecraftProfile, &'static str> {
    tauri::async_runtime::spawn_blocking(|| {
        let _guard = account_guard()?;
        nebula_core::accounts::refresh_active(&client_id()?).map_err(store_error)
    })
    .await
    .map_err(|_| "Accounts are unavailable.")?
}

const STATUS_EVENT: &str = "launcher-status-updated";

#[derive(Debug, Serialize)]
struct StatusError {
    code: &'static str,
    message: &'static str,
}

fn read_launcher_status<E>(
    reader: impl FnOnce() -> Result<MinecraftAccessStatus, E>,
    publish: impl FnOnce(MinecraftAccessStatus),
) -> Result<MinecraftAccessStatus, StatusError> {
    let status = reader().map_err(|_| StatusError {
        code: "STATUS_UNAVAILABLE",
        message: "Unable to refresh launcher status.",
    })?;
    publish(status);
    Ok(status)
}

#[tauri::command]
fn get_launcher_status<R: Runtime>(
    window: WebviewWindow<R>,
) -> Result<MinecraftAccessStatus, StatusError> {
    read_launcher_status(
        || Ok::<_, std::convert::Infallible>(minecraft_access_status()),
        |status| {
            let _ = window.emit_to(
                EventTarget::webview_window(window.label()),
                STATUS_EVENT,
                status,
            );
        },
    )
}

fn builder<R: Runtime>(builder: tauri::Builder<R>) -> tauri::Builder<R> {
    builder.invoke_handler(tauri::generate_handler![
        get_launcher_status,
        list_accounts,
        sign_in_account,
        select_account,
        remove_account,
        refresh_account
    ])
}

fn main() {
    builder(tauri::Builder::default())
        .run(tauri::generate_context!())
        .expect("failed to run Nebula Launcher");
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, json};
    use std::sync::mpsc;
    use tauri::{Listener, ipc, test, webview::InvokeRequest};

    #[test]
    fn scoped_status_command_returns_and_emits_the_core_snapshot() {
        let app = builder(test::mock_builder())
            .build(tauri::generate_context!())
            .unwrap();
        let window = tauri::WebviewWindowBuilder::new(&app, "main", Default::default())
            .build()
            .unwrap();
        let (sender, receiver) = mpsc::channel();
        window.listen(STATUS_EVENT, move |event| {
            sender
                .send(serde_json::from_str::<Value>(event.payload()).unwrap())
                .unwrap();
        });
        let response = test::get_ipc_response(
            &window,
            InvokeRequest {
                cmd: "get_launcher_status".into(),
                callback: ipc::CallbackFn(0),
                error: ipc::CallbackFn(1),
                url: "tauri://localhost".parse().unwrap(),
                body: ipc::InvokeBody::default(),
                headers: Default::default(),
                invoke_key: test::INVOKE_KEY.into(),
            },
        )
        .unwrap()
        .deserialize::<Value>()
        .unwrap();

        assert_eq!(
            response,
            serde_json::to_value(minecraft_access_status()).unwrap()
        );
        assert_eq!(receiver.try_recv().unwrap(), response);
    }

    #[test]
    fn capability_grants_only_local_status_and_event_subscription() {
        let capability: Value =
            serde_json::from_str(include_str!("../capabilities/main.json")).unwrap();
        let config: Value = serde_json::from_str(include_str!("../tauri.conf.json")).unwrap();
        assert_eq!(capability["identifier"], "main-status");
        assert_eq!(capability["windows"], json!(["main"]));
        assert!(capability.get("remote").is_none());
        assert_eq!(
            capability["permissions"],
            json!([
                "allow-get-launcher-status",
                "allow-list-accounts",
                "allow-sign-in-account",
                "allow-select-account",
                "allow-remove-account",
                "allow-refresh-account",
                "core:event:allow-listen",
                "core:event:allow-unlisten"
            ])
        );
        assert_eq!(
            config["app"]["security"]["capabilities"],
            json!(["main-status"])
        );
        assert_eq!(config["app"]["windows"][0]["dragDropEnabled"], false);
    }

    #[test]
    fn status_failure_discloses_only_the_public_error() {
        let error = read_launcher_status(
            || Err("synthetic internal diagnostic"),
            |_| panic!("failed reads must not publish status"),
        )
        .unwrap_err();
        assert_eq!(
            serde_json::to_value(error).unwrap(),
            json!({"code": "STATUS_UNAVAILABLE", "message": "Unable to refresh launcher status."})
        );
    }
}
