use actix_web::{App, HttpResponse, HttpServer, Responder, get, http::KeepAlive};

#[get("/metrics")]
async fn metrics_endpoint() -> impl Responder{
    match node_exporter::dump() {
        Ok((metrics, _err)) => {
            // XXX Log errors
            HttpResponse::Ok().body(metrics)
        },
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
