
pub fn tracing_init() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();
}

