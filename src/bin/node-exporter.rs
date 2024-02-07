use actix_web::{App, HttpResponse, HttpServer, Responder, get, http::KeepAlive};
use node_exporter::SystemState;

fn _metrics() -> node_exporter::Result<String> {
    let state = SystemState::new()?;
    node_exporter::dump(&state)
}

#[get("/metrics")]
async fn metrics_endpoint() -> impl Responder{
    match _metrics() {
        Ok(metrics) => HttpResponse::Ok().body(metrics),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
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
