extern crate embed_resource;

fn main() {
    embed_resource::compile("../assets/windows/tray.rc", embed_resource::NONE);

    // If debug mode, don't require admin. Admin allows us to get the exe_name of windows running outside of non-elevated user-space.
    if !cfg!(debug_assertions) {
        embed_resource::compile("../assets/windows/manifest.rc", embed_resource::NONE);
    }
}
