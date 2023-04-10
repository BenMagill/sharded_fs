mod files;
use std::{path::{PathBuf, Path}, sync::{Arc, Mutex}};

use actix_web::{App, HttpServer, web::{self, Data}};
use files::file_config;
use futures;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(move || {
        App::new()
            .service(web::scope("/files").configure(file_config))
    })
        .bind(("127.0.0.1", 8080))
        .unwrap()
        .run();

    // TODO: this would ideally be a lower level 
    let other_server = HttpServer::new(|| {
        App::new()
            // .service(web::scope("/files").configure(file_config))
    })
        .bind(("127.0.0.1", 8081))
        .unwrap()
        .run();
    
    futures::join!(server, other_server);

    return Ok(());
}

