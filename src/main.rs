use actix_web::{App, HttpResponse, HttpServer, Responder, get, http::KeepAlive};
use procfs::{CurrentSI, KernelStats};
use prometheus::{Encoder, TextEncoder};

mod metrics;

struct State {
    pub kernel_stats: KernelStats,
}

#[get("/metrics")]
async fn metrics_endpoint() -> impl Responder{
    let state = State { kernel_stats : KernelStats::current().unwrap()};

    metrics::update(&state);

    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();
    HttpResponse::Ok().body(buffer)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(metrics_endpoint)

    })
    .bind(("127.0.0.1", 9100))?
    .keep_alive(KeepAlive::Os)
    .workers(2)
    .run()
    .await
}
