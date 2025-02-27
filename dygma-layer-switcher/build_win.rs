extern crate embed_resource;

fn main() {
    let windows = "../assets/windows";

    embed(&format!("{}/icon.rc", windows));

    // Admin allows us to get the process of windows running in elevated user-space.
    if !cfg!(feature = "no-admin") {
        embed(&format!("{}/admin.rc", windows));
    }
}

fn embed(path: &str) {
    println!("cargo:rerun-if-changed={}", path);
    let _ = embed_resource::compile(path, embed_resource::NONE);
}
