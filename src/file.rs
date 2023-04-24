use crate::clients::*;
use crate::header::HeaderValue;
use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};
const NULL_CLIENT_HEADER: HeaderValue = HeaderValue::from_static("");

pub async fn delete_file(
    req: HttpRequest,
    file: web::Path<String>,
    clientes: web::Data<Vec<ClientConfig>>,
) -> HttpResponse {
    match is_client_valid(
        req.headers()
            .get("client")
            .unwrap_or(&NULL_CLIENT_HEADER)
            .to_str()
            .unwrap(),
        clientes,
    ) {
        true => exclude_file(&file),
        false => HttpResponse::BadRequest().body("Client not found"),
    }
}
pub fn exclude_file(filename: &str) -> HttpResponse {
    let mut path = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    path.push_str(r"\files\");
    path.push_str(&filename);

    let res = fs::remove_file(path);
    match res.is_ok() {
        true => HttpResponse::Ok().body(
            serde_json::to_value(FileResponse {
                success: true,
                message: "File deleted".to_string(),
                file: filename.to_owned(),
            })
            .unwrap()
            .to_string(),
        ),
        false => HttpResponse::InternalServerError().body(
            serde_json::to_value(FileResponse {
                success: false,
                message: res.err().unwrap().to_string(),
                file: filename.to_owned(),
            })
            .unwrap()
            .to_string(),
        ),
    }
}

pub async fn post_file(
    req: HttpRequest,
    clientes: web::Data<Vec<ClientConfig>>,
    body: web::Bytes,
) -> HttpResponse {
    let asd = &clientes[0];

    match is_client_valid(
        req.headers()
            .get("client")
            .unwrap_or(&NULL_CLIENT_HEADER)
            .to_str()
            .unwrap(),
        clientes,
    ) {
        true => create_file(body),
        false => HttpResponse::BadRequest().body("Client not found"),
    }
}
#[derive(Serialize, Deserialize)]
struct FileResponse {
    success: bool,
    message: String,
    file: String,
}

fn is_client_valid(client: &str, clientes: web::Data<Vec<ClientConfig>>) -> bool {
    let mut ret = false;
    if client == "" {
        ret = false;
    } else {
        for cliente in clientes.iter() {
            if cliente.id == client {
                ret = true;
            } else {
                ret = false;
            }
        }
    }
    ret
}

pub fn create_file(content: web::Bytes) -> HttpResponse {
    let filename = get_file_name();
    let mut path = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    path.push_str(r"\files\");
    path.push_str(&filename);

    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(e) => {
            println!("Fail during creating file: {:?}", e);
            return HttpResponse::InternalServerError().body(
                serde_json::to_value(FileResponse {
                    success: false,
                    message: "Error during creating file".to_string(),
                    file: "".to_string(),
                })
                .unwrap()
                .to_string(),
            );
        }
    };

    match file.write_all(&content) {
        Ok(_) => log::info!("File saved!"),
        Err(e) => {
            return HttpResponse::InternalServerError().body(
                serde_json::to_value(FileResponse {
                    success: false,
                    message: "Error during writing file - ".to_string() + e.to_string().as_str(),
                    file: "".to_string(),
                })
                .unwrap()
                .to_string(),
            );
        }
    };
    HttpResponse::Ok().body(
        serde_json::to_value(FileResponse {
            success: true,
            message: "File saved".to_string(),
            file: filename,
        })
        .unwrap()
        .to_string(),
    )
}

pub fn get_file_name() -> String {
    let now = SystemTime::now();
    let since_epoch = now
        .duration_since(UNIX_EPOCH)
        .expect("Failed to get duration since epoch");

    let name = format!(
        "{:02}{:02}{:02}{:02}{:06}",
        since_epoch.as_secs() / 86400,
        (since_epoch.as_secs() % 86400) / 3600,
        (since_epoch.as_secs() % 3600) / 60,
        (since_epoch.as_secs() % 60),
        since_epoch.subsec_nanos() / 1000,
    );
    name
}
