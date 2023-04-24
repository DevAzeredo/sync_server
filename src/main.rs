mod clients;
mod file;
use actix_cors::Cors;
use actix_files::NamedFile;
use actix_web::{
    http::{self, header, Method, StatusCode},
    middleware, web, App, Either, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use clients::ClientConfig;
use clients::ClientesConfigManager;
use lazy_static::lazy_static;
use std::io;

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

async fn handle_post_file(
    req: HttpRequest,
    arquivos: web::Data<Vec<ClientConfig>>,
    body: web::Bytes,
) -> HttpResponse {
    file::post_file(req, arquivos, body).await
}
async fn handle_delete_file(
    req: HttpRequest,
    file: web::Path<String>,
    clientes: web::Data<Vec<ClientConfig>>,
) -> HttpResponse {
    file::delete_file(req, file, clientes).await
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
        let clientes: Vec<ClientConfig> = CLIENTES.clone();

        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(web::PayloadConfig::new(10_485_760))
            .app_data(web::Data::new(clientes))
            .service(web::resource("/files").route(web::post().to(handle_post_file)))
            .service(web::resource("/files/{file_name}").route(web::delete().to(handle_delete_file)))
            // default
            .default_service(web::to(default_handler))
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}
