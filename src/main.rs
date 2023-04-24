mod clients;
mod file;
use crate::clients::ClientConfig;
use actix_cors::Cors;
use actix_files::NamedFile;
use clients::ClientesConfigManager;
use lazy_static::lazy_static;
use std::io;

use actix_web::{
    error,
    http::{
        self,
        header::{self},
        Method, StatusCode,
    },
    middleware, web, App, Either, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};

async fn default_handler(req_method: Method) -> Result<impl Responder> {
    match req_method {
        Method::GET => {
            let file = NamedFile::open("static/404.html")?
                .customize()
                .with_status(StatusCode::NOT_FOUND);
            Ok(Either::Left(file))
        }
        _ => Ok(Either::Right(HttpResponse::MethodNotAllowed().finish())),
    }
}

async fn handle_post(
    req: HttpRequest,
    arquivos: web::Data<Vec<ClientConfig>>,
    body: web::Bytes,
) -> HttpResponse {
    file::post_file(req, arquivos, body).await
}

lazy_static! {
    static ref CLIENTES: Vec<ClientConfig> =
        ClientesConfigManager::new().get_clientes_by_ini().to_vec();
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin().allowed_headers(vec![
            http::header::AUTHORIZATION,
            http::header::ACCEPT,
            http::header::CONTENT_TYPE,
        ]);
        let clientes = CLIENTES.clone();

        App::new()
            // enable automatic response compression - usually register this first
            .wrap(middleware::Compress::default())
            // enable logger - always register Actix Web Logger middleware last
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(web::PayloadConfig::new(10_485_760))
            .app_data(web::Data::new(clientes))
            .service(web::resource("/arquivos").route(web::post().to(handle_post)))
            .service(
                web::resource("/test").to(|req: HttpRequest| match *req.method() {
                    Method::GET => HttpResponse::Ok(),
                    Method::POST => HttpResponse::MethodNotAllowed(),
                    _ => HttpResponse::NotFound(),
                }),
            )
            .service(web::resource("/error").to(|| async {
                error::InternalError::new(
                    io::Error::new(io::ErrorKind::Other, "test"),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
            }))
            // redirect
            .service(
                web::resource("/").route(web::get().to(|req: HttpRequest| async move {
                    println!("{req:?}");
                    HttpResponse::Found()
                        .insert_header((header::LOCATION, "static/welcome.html"))
                        .finish()
                })),
            )
            // default
            .default_service(web::to(default_handler))
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}
