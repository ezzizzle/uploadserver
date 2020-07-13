use actix_multipart::Multipart;
use actix_web::http::StatusCode;
use actix_web::{guard, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use async_std::prelude::*;
use futures::{StreamExt, TryStreamExt};
use std::env;
use uploadserver::Config;

async fn save_file(
    config: web::Data<Config<'_>>,
    mut payload: Multipart,
    req: HttpRequest,
    // form: Form<UploadForm>,
) -> Result<HttpResponse, Error> {
    let upload_path = match req.headers().get("upload_path") {
        Some(path_value) => path_value.to_str().unwrap(),
        None => {
            return Ok(HttpResponse::build(StatusCode::BAD_REQUEST)
                .content_type("application/json; charset=utf-8")
                .body("{\"error\": \"No upload_path header\"}"))
        }
    };

    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let _content_type = field
            .content_disposition()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
        // let filename = content_type
        //     .get_filename()
        //     .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
        let filepath = format!(
            "{}/{}",
            config.file_directory,
            upload_path // sanitize_filename::sanitize(&filename)
        );
        println!("Writing to {}", filepath);
        let filepath = std::path::Path::new(&filepath);
        let parent_dirs = filepath.parent().unwrap();
        async_std::fs::create_dir_all(parent_dirs).await?;
        let mut f = async_std::fs::File::create(filepath).await?;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).await?;
        }
    }

    return Ok(HttpResponse::build(StatusCode::OK)
        .content_type("application/json; charset=utf-8")
        .body(format!(
            "{{\"url\": \"{}{}\"}}",
            config.base_url, upload_path
        )));
}

fn index() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/" method="post" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <input type="submit" value="Submit"></button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok().body(html)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_server=info,actix_web=info");

    let config = Config::new();

    let ip = format!("0.0.0.0:{}", config.port);

    HttpServer::new(move || {
        App::new()
            .data(config)
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/").route(web::get().to(index)).route(
                    web::post()
                        .guard(guard::Header("token", config.token))
                        .to(save_file),
                ),
            )
    })
    .bind(ip)?
    .run()
    .await
}
