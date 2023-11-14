extern crate embed_resource;

fn main() {
    embed_resource::compile("../assets/windows/tray.rc", embed_resource::NONE);

    // Admin allows us to get the exe_name of windows running in elevated user-space.
    if !cfg!(feature = "no-admin") && !cfg!(debug_assertions) {
        embed_resource::compile("../assets/windows/manifest.rc", embed_resource::NONE);
    }
}
