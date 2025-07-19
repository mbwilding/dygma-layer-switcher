fn main() {
    tauri_build::build();

    #[cfg(all(windows, not(feature = "no-admin")))]
    {
        embed_resource::compile("assets/windows/admin.rc");
    }
}
