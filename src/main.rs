mod files;
use std::{fs::File, io::{Write, Read}, future};
use std::fs;
use actix_web::{get, App, HttpServer, HttpResponse, Responder, rt, web};
use files::file_config;
use futures;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .service(web::scope("/files").configure(file_config))
    })
        .bind(("127.0.0.1", 8080))
        .unwrap()
        .run();

    // TODO: this would ideally be a lower level 
    let other_server = HttpServer::new(|| {
        App::new()
            .service(web::scope("/files").configure(file_config))
    })
        .bind(("127.0.0.1", 8081))
        .unwrap()
        .run();
    
    futures::try_join!(server, other_server);

    return Ok(());
}

