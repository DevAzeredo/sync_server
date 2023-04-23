use actix_web::{web, HttpRequest, HttpResponse};
use std::fs::File;
use std::io::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn post_file(req: HttpRequest, body: web::Bytes) -> HttpResponse {
    match is_client_valid(req.headers().get("client").unwrap().to_str().unwrap()) {
        true => create_file(body),
        false => HttpResponse::BadRequest().body("Client not found"),
    }
}

fn is_client_valid(client: &str) -> bool {
    if client == "" {
        return false;
    } else {
        return true;
    }
}

pub fn create_file(content: web::Bytes) -> HttpResponse {
    let path = std::env::current_exe().unwrap_or_default();
    let filename = get_file_name();

    let mut file = match File::create(path.join(r"\files\").join(&filename)) {
        Ok(file) => file,
        Err(e) => {
            println!("Fail during creating file: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    match file.write_all(&content) {
        Ok(_) => println!("File saved!"),
        Err(e) => {
            println!("Error during writing file: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::Ok().body(format!("File {} sucessfully saved", filename))
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
