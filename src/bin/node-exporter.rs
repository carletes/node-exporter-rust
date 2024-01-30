use actix_web::{App, HttpResponse, HttpServer, Responder, get, http::KeepAlive};
use node_exporter::SystemState;
use procfs::{CurrentSI, KernelStats};

#[get("/metrics")]
async fn metrics_endpoint() -> impl Responder{
    let state = SystemState { kernel_stats : KernelStats::current().unwrap()};

    let metrics = node_exporter::dump(&state);
    HttpResponse::Ok().body(metrics)
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
