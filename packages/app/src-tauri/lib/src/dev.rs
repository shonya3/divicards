use tracing_subscriber::fmt::format::FmtSpan;

pub fn init_tracing() {
    let log_filter = match std::env::var("RUST_LOG") {
        Ok(filter) => filter,
        Err(_) => String::from("app=debug,tauri=debug,lib=info,divi=info,googlesheets=info"),
    };

    println!("tracing mode: {log_filter}");

    let span_events = match std::env::var("RUST_SPAN_EVENTS") {
        Ok(val) if val.eq_ignore_ascii_case("close") => FmtSpan::CLOSE,
        Ok(val) if val.eq_ignore_ascii_case("full") => FmtSpan::FULL,
        _ => FmtSpan::NONE,
    };

    tracing_subscriber::fmt()
        .with_env_filter(log_filter)
        .with_span_events(span_events)
        .init();
}
