extern crate embed_resource;

fn main() {
    embed_resource::compile("../assets/windows/tray.rc", embed_resource::NONE);
}
