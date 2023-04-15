use std::{fs::{self, rename, copy}, path::{Path, PathBuf}, io::Error, sync::{Arc, Mutex}};

use actix_multipart::{form::{MultipartForm, text::Text, tempfile::{TempFile, TempFileConfig}}};
use actix_web::{web::{self, Data}, Responder, HttpRequest};

use crate::librarian::Library;

struct FilesConfig {
    storage_folder: PathBuf,
    temp_folder: PathBuf,
}

#[derive(Debug, MultipartForm)]
struct FileUpload {
    file_name: Text<String>,
    file: TempFile,
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
        web::resource("/read/{file_name}")
            .route(web::get().to(read))
    );
    cfg.service(
        web::resource("/write")
            .app_data(multipart_data)
            .route(web::post().to(write))
    );
}

async fn read(req: HttpRequest, path: web::Path<String>) -> impl Responder {
    let config = req.app_data::<Data<FilesConfig>>()
        .unwrap();
    let file_name = path.into_inner();
    
    let result = read_from_file(get_path(config, file_name.as_str()));

    let lib = &req.app_data::<Arc<Mutex<Library>>>()
        .unwrap().lock().unwrap().files;
    
    for (key) in lib.iter() {
        println!("{}, {:?}", key.0, key.1);
    }

    return match result {
        Ok(contents) => { contents },
        Err(_) => { "Could not find file".as_bytes().to_vec() },
    };
}

async fn write(req: HttpRequest, form: MultipartForm<FileUpload>) -> impl Responder {
    let config = req.app_data::<Data<FilesConfig>>()
        .unwrap();

    let mut lib = req.app_data::<Arc<Mutex<Library>>>()
        .unwrap().lock().unwrap();
    
    let temp_name = form.file.file.path();
    let filename = form.file_name.0.as_str();
    let path = get_path(config, filename);

    return match move_file(temp_name, path) {
        Ok(no_copy) => {
            if no_copy { println!("Could not move file, had to copy") };

            // Add file to Library
            lib.add_object(filename.to_string(), true);

            "Ok"
        },
        Err((e1, e2)) => {
            println!("Error from move: {}", e1);
            println!("Error from copy: {}", e2);
            "Error"
        }
    };
}

fn get_path(config: &Data<FilesConfig>, filename: &str) -> PathBuf {
    return config.storage_folder.join(filename);
}

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
