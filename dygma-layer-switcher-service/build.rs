extern crate embed_resource;

fn main() {
    embed_resource::compile("../assets/tray-windows.rc", embed_resource::NONE);
}
