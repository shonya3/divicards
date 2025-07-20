use tracing_subscriber::fmt::format::FmtSpan;

pub fn init_tracing() {
    let log_filter = match std::env::var("RUST_LOG") {
        Ok(filter) => filter,
        Err(_) => String::from("app=debug,tauri=debug,lib=debug,divi=debug,googlesheets=debug"),
    };

    println!("tracing mode: {log_filter}");

    tracing_subscriber::fmt()
        .with_env_filter(log_filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();
}
