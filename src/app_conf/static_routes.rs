use actix_files;
use actix_web::{HttpRequest, Result, HttpResponse, Scope, web};
use actix_web::http::{StatusCode};
use std::{fs, path::PathBuf};

const AUTHORIZED_STATIC_PATHS: & [&str] = &[
    "ask_password_reset.html", 
    "login.html",
    "reset_password.html",
    "email_confirmation.html",
];

pub fn get_all() -> Scope {
    web::scope("/static")
        .service(web::resource("/{html_file_path}")
            .route(web::get().to(static_file_http_response)))
        .service(web::resource("/assets/{filename:.*}")
            .route(web::get().to(file)))
}

async fn file(req: HttpRequest) -> Result<actix_files::NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    let mut prefix = "static/assets/".to_owned();
    if let Some(str_path) = path.to_str() {
        prefix.push_str(str_path);
    }

    Ok(actix_files::NamedFile::open(prefix)?)
}

async fn static_file_http_response(html_file_path: web::Path<String>) -> HttpResponse {
    // check if it's an authorized path
    let path = html_file_path.to_string();
    let mut find = false;
    for p in AUTHORIZED_STATIC_PATHS {
        if p.to_string() == path {
            find = true;
            break;
        }
    }

    let error_closure = move || {
        let error_message = format!("Unknown path: {}", path);

        return HttpResponse::build(StatusCode::NOT_FOUND)
            .content_type("text/html; charset=utf-8")
            .body(error_message)
    };

    if !find {
        error_closure()
    } else {
        let extension = html_file_path.split(".").last().unwrap_or_default();

        if extension.len() == 0  {
            return error_closure();
        }
        let full_path = format!("static/html/{}", html_file_path);

        match fs::read_to_string(full_path) {
            Ok(contents) => {
                let mut content_type = "text/".to_owned();

                if extension != "js" {
                    content_type.push_str(&extension);
                } else {
                    content_type.push_str("javascript");
                }
                
                content_type.push_str("; charset=utf-8");
                let content_type = content_type;

                HttpResponse::build(StatusCode::OK)
                    .content_type(content_type)
                    .body(contents)
            }, Err(e) => {
                println!("{}", e);
                error_closure()
            }
        }      
    }
}