use actix_web::{HttpServer, App, HttpRequest, HttpResponse, Result, http::header};
use actix_web::web;
use actix_web::error::Error as ActixError;
use actix_files as fs;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::borrow::ToOwned;

pub async fn index(_req: HttpRequest) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/index.html")?)
}

pub async fn static_files_manual_async(req: HttpRequest) -> Result<HttpResponse, ActixError> {
    let file_path_str = format!(".{}", req.path().replace("static2", "static"));
    let file_path = Path::new(&file_path_str);
    let file_name = file_path.file_name().unwrap().to_str().unwrap().to_owned();
    let file_ext = file_path.extension().unwrap().to_str().unwrap().to_owned();
    let file_mime = fs::file_extension_to_mime(&file_ext);
    let content_type = format!("{}/{}", file_mime.type_(), file_mime.subtype());

    let mut f = File::open(file_path_str).unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();

    Ok(HttpResponse::Ok()
        .header(header::CONTENT_DISPOSITION, header::ContentDisposition {
            disposition: header::DispositionType::Inline,
            parameters: vec![
                header::DispositionParam::Filename(file_name.to_owned())
            ]
        })
        .content_type(content_type)
        .content_length(buffer.len() as u64)
        .body(buffer))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(web::resource("/")
                .name("index")
                .route(web::get().to(index))
            )
            .service(fs::Files::new("/static", "./static"))
            .service(web::resource("/static2/{tail:.*}")
                .route(web::get().to(static_files_manual_async))
            )
    }).bind("0.0.0.0:8081")
        .expect("Cannot bind 0.0.0.0:8081")
        .run().await
}
