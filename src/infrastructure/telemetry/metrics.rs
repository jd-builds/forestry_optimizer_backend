use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};
use std::net::SocketAddr;
use actix_web::{web, App, HttpServer, HttpResponse};
use tracing::info;
use sysinfo::{SystemExt, ProcessExt};

/// Initialize the metrics system
pub async fn init_metrics(metrics_addr: SocketAddr) -> std::io::Result<PrometheusHandle> {
    // Create Prometheus handle
    let handle = setup_metrics_recorder()?;
    let metrics_handle = handle.clone();

    // Start metrics server
    tokio::spawn(async move {
        let metrics_app = move || {
            let handle = handle.clone();
            App::new()
                .route("/metrics", web::get().to(move || {
                    let metrics = handle.render();
                    async move { HttpResponse::Ok().body(metrics) }
                }))
        };

        info!("Starting metrics server on {}", metrics_addr);
        HttpServer::new(metrics_app)
            .bind(metrics_addr)
            .expect("Failed to bind metrics server")
            .run()
            .await
            .expect("Metrics server failed");
    });

    Ok(metrics_handle)
}

/// Set up the metrics recorder with custom configuration
fn setup_metrics_recorder() -> std::io::Result<PrometheusHandle> {
    const EXPONENTIAL_SECONDS: &[f64] = &[
        0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
    ];

    PrometheusBuilder::new()
        // Record http requests duration with custom buckets
        .set_buckets_for_metric(
            Matcher::Full("http_requests_duration_seconds".to_string()),
            EXPONENTIAL_SECONDS,
        )
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?
        // Install globally
        .install_recorder()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}

/// Register common application metrics
pub fn register_metrics() {
    // System metrics
    metrics::gauge!("process_start_time_seconds", unix_timestamp() as f64);
    metrics::gauge!("process_cpu_cores", num_cpus::get() as f64);
    
    // These will be updated by the metrics collection task
    metrics::gauge!("process_memory_bytes", 0.0);
    metrics::gauge!("process_cpu_usage_percentage", 0.0);
}

/// Start a background task to collect system metrics
pub fn spawn_metrics_collector() {
    tokio::spawn(async move {
        let mut sys = sysinfo::System::new();
        sys.refresh_all();
        let pid = sysinfo::get_current_pid().expect("Failed to get PID");

        loop {
            if let Some(process) = sys.process(pid) {
                metrics::gauge!("process_memory_bytes", process.memory() as f64);
                metrics::gauge!("process_cpu_usage_percentage", process.cpu_usage() as f64);
            }
            tokio::time::sleep(std::time::Duration::from_secs(15)).await;
        }
    });
}

fn unix_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
} 