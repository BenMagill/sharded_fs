mod files;
mod librarian;


use actix_web::{App, HttpServer, web::{self}};
use files::file_config;
use futures;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(move || {
        App::new()
            .service(web::scope("/files").configure(file_config))
            .service(web::resource("/health")
                .route(web::get().to(|| async {"ok"}))
            )
    })
        .bind(("127.0.0.1", 8080))
        .unwrap()
        .run();

    // TODO: this would ideally be a lower level not http
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

