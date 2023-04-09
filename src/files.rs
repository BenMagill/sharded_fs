use std::{fs::{self, rename, copy}, path::{Path, PathBuf}, env::temp_dir, time::Duration};

use actix_multipart::{form::{MultipartForm, text::Text, tempfile::{TempFile, TempFileConfig, TempFileError}}, Multipart};
use actix_rt::time::sleep;
use actix_web::{web, Responder, HttpRequest, get};
use futures::StreamExt;

async fn index() -> impl Responder {
    "Hello!"
}

// TODO: stream result for large files? 
async fn read(path: web::Path<String>) -> impl Responder {
    let file_name = path.into_inner();

    return match read_from_file(file_name.as_str()) {
        Ok(contents) => { contents },
        Err(_) => { "Could not find file".as_bytes().to_vec() },
    };
}

#[derive(Debug, MultipartForm)]
struct FileUpload {
    file_name: Text<String>,
    file: TempFile,
}

async fn write(form: MultipartForm<FileUpload>) -> impl Responder {
    
    println!("{}", form.file_name.0);
    println!("{}", form.file.file.path().to_str().unwrap());
    let temp_name = form.file.file.path();
    let path = get_path(form.file_name.0.as_str());

    return match rename(temp_name, &path) {
        Ok(_) => { "ok" },
        Err(e) => { 
            println!("{}", e.to_string());
            println!("Having to copy file instead of move");
            return match copy(temp_name, &path) {
                Ok(_) => { "ok" },
                Err(e) => {
                    println!("{}", e.to_string());
                    "error"
                }
            };
        },
    };
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
    let multipart_data = TempFileConfig::default()
        .directory(Path::new("temp").to_path_buf());

    cfg.service(
        web::resource("/")
            .route(web::get().to(index))
    );
    cfg.service(
        web::resource("/read/{file_name}")
            .route(web::get().to(read))
    );
    cfg.service(
        web::resource("/write")
            .app_data(multipart_data)
            .route(web::post().to(write))
    );
}