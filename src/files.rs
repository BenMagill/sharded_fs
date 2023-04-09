use std::{fs, path::{Path, PathBuf}};

use actix_web::{web, Responder, HttpRequest, get};

async fn index() -> impl Responder {
    "Hello!"
}

async fn read(path: web::Path<(String)>) -> impl Responder {
    let (file_name) = path.into_inner();
    println!("{}", file_name);
    return "Ok";
}

fn get_path(filename: &str) -> PathBuf {
    return Path::new("files").join(filename);
}

fn write_to_file(filename: &str, buf: &[u8]) -> Result<(), std::io::Error> {
    // TODO: safely create
    return fs::write(get_path(filename), buf);
}

fn read_from_file(filename: &str) -> std::io::Result<Vec<u8>> {
    // TODO: safely read
    let contents = fs::read(get_path(filename));
    return contents;
}

pub fn file_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(index))
    );
    cfg.service(
        web::resource("/read/{file_name}")
            .route(web::get().to(read))
    );
}