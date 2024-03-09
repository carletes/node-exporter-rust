use actix_web::{App, HttpResponse, HttpServer, Responder, get, http::KeepAlive, http::header::ContentType};
use log::error;

#[get("/metrics")]
async fn metrics_endpoint() -> impl Responder{
    match node_exporter::dump() {
        Ok((metrics, _err)) => {
            // XXX Log errors
            HttpResponse::Ok()
                .content_type(ContentType::plaintext())
                .body(metrics)
        },
        Err(err) => {
            error!("Error dumping metrics: {:?}", err);
            HttpResponse::InternalServerError()
                .content_type(ContentType::plaintext())
                .body(format!("{:?}", err))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| {
        App::new().service(metrics_endpoint)

    })
    .bind(("0.0.0.0", 9100))?
    .keep_alive(KeepAlive::Os)
    .workers(1)
    .run()
    .await
}
