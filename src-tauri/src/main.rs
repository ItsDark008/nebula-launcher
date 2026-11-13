use nebula_core::{MinecraftAccessStatus, minecraft_access_status};
use serde::Serialize;
use tauri::{Emitter, EventTarget, Runtime, WebviewWindow};

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
    builder.invoke_handler(tauri::generate_handler![get_launcher_status])
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
