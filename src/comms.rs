use actix_web::web;

pub fn comms_config(cfg: &mut web::ServiceConfig) {

    // Recieve connection request and return other addresses
    cfg.service(
        web::resource("/connect")
            .route(web::get().to(read))
    );
    
    // Handle changes to other nodes
    // node added
    // file added
    cfg.service(
        web::resource("/resync")
            .route(web::get().to(read))
    );
 
    // Read file to other
    cfg.service(
        web::resource("/read")
            .route(web::get().to(read))
    );
}
