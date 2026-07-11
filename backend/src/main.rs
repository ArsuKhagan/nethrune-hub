use axum::{routing::get, Json, Router};
use serde::Serialize;

#[derive(Serialize)]
struct SystemInfo {
    hostname: String,
    os_name: String,
    kernel_version: String,
    architecture: String,
    uptime: u64,
}

async fn system_info() -> Json<SystemInfo> {
    // Переменная sys больше не нужна, так как все вызовы теперь статические.
    Json(SystemInfo {
        hostname: sysinfo::System::host_name().unwrap_or_default(),
        os_name: sysinfo::System::name().unwrap_or_default(),
        kernel_version: sysinfo::System::kernel_version().unwrap_or_default(),
        architecture: std::env::consts::ARCH.to_string(),
        uptime: sysinfo::System::uptime(),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/api/system/info", get(system_info));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap(); 
}
