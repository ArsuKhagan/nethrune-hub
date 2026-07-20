use axum::{routing::get, Json, Router};
use serde::Serialize;
use std::collections::HashSet;
use sysinfo::{Disks, System};
use tower_http::cors::CorsLayer;

#[derive(Serialize)]
struct SystemInfo {
    hostname: String,
    os_name: String,
    kernel_version: String,
    architecture: String,
    uptime: u64,
}

async fn system_info() -> Json<SystemInfo> {
    Json(SystemInfo {
        hostname: sysinfo::System::host_name().unwrap_or_default(),
        os_name: sysinfo::System::name().unwrap_or_default(),
        kernel_version: sysinfo::System::kernel_version().unwrap_or_default(),
        architecture: std::env::consts::ARCH.to_string(),
        uptime: sysinfo::System::uptime(),
    })
}

#[derive(Serialize)]
struct DiskUsage {
    name: String,
    mount_point: String,
    total_space: u64,
    available_space: u64,
}

#[derive(Serialize)]
struct SystemResources {
    cpu_usage: f32,
    memory_total: u64,
    memory_used: u64,
    swap_total: u64,
    swap_used: u64,
    disks: Vec<DiskUsage>,
}

const VIRTUAL_FILESYSTEMS: &[&str] = &[
    "tmpfs",
    "devtmpfs",
    "proc",
    "sysfs",
    "overlay",
    "squashfs",
    "cgroup",
    "cgroup2",
    "debugfs",
    "tracefs",
    "pstore",
    "mqueue",
    "hugetlbfs",
    "fuse.portal",
    "autofs",
    "binfmt_misc",
    "securityfs",
    "configfs",
    "ramfs",
    "devpts",
];

async fn system_resources() -> Json<SystemResources> {
    let mut sys = System::new_all();
    sys.refresh_cpu_usage();
    tokio::time::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL).await;
    sys.refresh_cpu_usage();
    sys.refresh_memory();

    let mut seen = HashSet::new();
    let disks = Disks::new_with_refreshed_list()
        .list()
        .iter()
        .filter(|disk| {
            let fs = disk.file_system().to_string_lossy().to_lowercase();
            !VIRTUAL_FILESYSTEMS.contains(&fs.as_str())
        })
        .filter(|disk| seen.insert(disk.name().to_os_string()))
        .map(|disk| DiskUsage {
            name: disk.name().to_string_lossy().to_string(),
            mount_point: disk.mount_point().to_string_lossy().to_string(),
            total_space: disk.total_space(),
            available_space: disk.available_space(),
        })
        .collect();

    Json(SystemResources {
        cpu_usage: sys.global_cpu_usage(),
        memory_total: sys.total_memory(),
        memory_used: sys.used_memory(),
        swap_total: sys.total_swap(),
        swap_used: sys.used_swap(),
        disks,
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/api/system/info", get(system_info))
        .route("/api/system/resources", get(system_resources))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}