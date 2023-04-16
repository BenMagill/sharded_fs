mod files;
mod librarian;
mod node_manager;
mod comms;


use std::sync::{Arc, Mutex};

use actix_web::{App, HttpServer, web::{self, Data}};
use files::file_config;
use futures;

use crate::comms::comms_config;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let lib = Arc::new(Mutex::new(librarian::Library::new()));

    let lib_server = lib.clone();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(lib_server.clone())
            .service(web::scope("/files").configure(file_config))
            .service(web::scope("/internals").configure(comms_config))
            .service(web::resource("/health")
                .route(web::get().to(|| async {"ok"}))
            )
    })
        .bind(("127.0.0.1", 8080))
        .unwrap()
        .run().await;

    // TODO: this would ideally be a lower level not http
    // let other_server = HttpServer::new(|| {
    //     App::new()
    //         // .service(web::scope("/files").configure(file_config))
    // })
    //     .bind(("127.0.0.1", 8081))
    //     .unwrap()
    //     .run();
    
    // futures::join!(server, other_server);

    return Ok(());
}

