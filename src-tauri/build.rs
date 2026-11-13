fn main() {
    tauri_build::try_build(
        tauri_build::Attributes::new()
            .app_manifest(tauri_build::AppManifest::new().commands(&["get_launcher_status"])),
    )
    .expect("failed to build the Tauri application context");
}
