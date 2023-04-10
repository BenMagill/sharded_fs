use std::{fs::{self, rename, copy}, path::{Path, PathBuf}, io::Error};

use actix_multipart::{form::{MultipartForm, text::Text, tempfile::{TempFile, TempFileConfig}}};
use actix_web::{web::{self, Data}, Responder, HttpRequest};

struct FilesConfig {
    storage_folder: PathBuf,
    temp_folder: PathBuf,
}

async fn index() -> impl Responder {
    "Hello!"
}

// TODO: stream result for large files? 
async fn read(req: HttpRequest, path: web::Path<String>) -> impl Responder {
    let config = req.app_data::<Data<FilesConfig>>()
        .unwrap();
    let file_name = path.into_inner();
    
    let result = read_from_file(get_path(config, file_name.as_str()));
    return match result {
        Ok(contents) => { contents },
        Err(_) => { "Could not find file".as_bytes().to_vec() },
    };
}

#[derive(Debug, MultipartForm)]
struct FileUpload {
    file_name: Text<String>,
    file: TempFile,
}

async fn write(req: HttpRequest, form: MultipartForm<FileUpload>) -> impl Responder {
    let config = req.app_data::<Data<FilesConfig>>()
        .unwrap();
    
    println!("{}", form.file_name.0);
    println!("{}", form.file.file.path().to_str().unwrap());
    let temp_name = form.file.file.path();
    let path = get_path(config, form.file_name.0.as_str());

    return match move_file(temp_name, path) {
        Ok(no_copy) => {
            if no_copy { println!("Could not move file, had to copy") };
            "Ok"
        },
        Err((e1, e2)) => {
            println!("Error from move: {}", e1);
            println!("Error from copy: {}", e2);
            "Error"
        }
    };
}

// fn get_temp_folder() -> PathBuf {
//     Path::new("temp").to_owned()
// }

// fn get_storage_folder() -> PathBuf {
//     Path::new("files").to_owned()
// }

fn get_path(config: &Data<FilesConfig>, filename: &str) -> PathBuf {
    return config.storage_folder.join(filename);
}

/**
 * Ok(bool): true when used fs move, false when had to copy 
 */
fn move_file<T: AsRef<Path>, S: AsRef<Path>>(from: T, to: S) -> Result<bool, (Error, Error)> {
    return match rename(&from, &to) {
        Ok(_) => { Ok(true) },
        Err(e) => { 
            return match copy(&from, &to) {
                Ok(_) => { Ok(false) },
                Err(e2) => {
                    Err((e, e2))
                }
            };
        },
    };
}

fn read_from_file(path: PathBuf) -> std::io::Result<Vec<u8>> {
    let contents = fs::read(path);
    return contents;
}

pub fn file_config(cfg: &mut web::ServiceConfig) {
    let config = FilesConfig {
        storage_folder: Path::new("files").to_owned(),
        temp_folder: Path::new("temp").to_owned()
    };
    
    let multipart_data = TempFileConfig::default()
        .directory(&config.temp_folder);

    let a = Data::new(config);

    cfg.app_data(Data::clone(&a));
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