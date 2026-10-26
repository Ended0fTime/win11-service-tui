fn main() {
    // Only compile the resource file if we are targeting Windows
    if std::env::var("CARGO_CFG_WINDOWS").is_ok() {
        embed_resource::compile("resources.rc", embed_resource::NONE);
    }
}
