fn main() {
    tauri_build::try_build(tauri_build::Attributes::new().app_manifest(
        tauri_build::AppManifest::new().commands(&[
            "get_launcher_status",
            "list_accounts",
            "sign_in_account",
            "select_account",
            "remove_account",
            "refresh_account",
        ]),
    ))
    .expect("failed to build the Tauri application context");
}
