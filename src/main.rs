#[macro_use]
extern crate lazy_static;

use std::io::Cursor;

use actix_web::{get, App, HttpResponse, HttpServer, Responder, HttpRequest, http::Uri, Result, web::Path};
use image::ImageFormat;
use maud::html;
use regex::Regex;
use serde::Deserialize;
use dotenv::dotenv;

mod image_generator;

lazy_static! {
    static ref UA_REGEX: Regex = Regex::new(r"bot|facebook|embed|got|firefox/92|firefox/38|curl|wget|go-http|yahoo|generator|whatsapp|preview|link|proxy|vkshare|images|analyzer|index|crawl|spider|python|cfnetwork|node").unwrap();
}

#[derive(Deserialize)]
struct SrcPath {
    author: String,
    repository: String,
    branch: String,
    path: String
}


#[get("/image/{author}/{repository}/{branch}/{path:.*}")]
async fn get_source_image(_req: HttpRequest, path: Path<SrcPath>) -> Result<impl Responder> {
    let code_uri = Uri::builder()
        .scheme("https")
        .authority("raw.githubusercontent.com")
        .path_and_query(format!("/{}/{}/{}/{}", path.author, path.repository, path.branch, path.path))
        .build()?;

    if let Ok(request) = reqwest::get(code_uri.to_string()).await {
        if let Some(user_agent_string) = request.headers().get("User-Agent").and_then(|user_agent| { user_agent.to_str().ok() }) {
            if !user_agent_string.contains("text/plain") {
                return Ok(HttpResponse::TemporaryRedirect().insert_header(("Location", code_uri.to_string())).finish());
            }
            else if let Ok(src_code) = request.text().await {
                let mut buffer = Vec::new();
                image_generator::generate_src_image(&src_code).write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png).unwrap();
                return Ok(HttpResponse::Ok()
                    .content_type("image/png")
                    .body(buffer));
            }
        }
    }

    Ok(HttpResponse::NotFound().body("Unable to fetch code..."))
}

#[get("/{author}/{repository}/blob/{branch}/{path:.*}")]
async fn get_open_graph(req: HttpRequest, path: Path<SrcPath>) -> impl Responder {
    let canon_url = format!("https://github.com{}", req.uri());

    if let Some(user_agent_string) = req.headers().get("User-Agent").and_then(|user_agent| { user_agent.to_str().ok() }) {
        if UA_REGEX.is_match(&user_agent_string.to_lowercase()) {
            let og_image = format!("/image/{}/{}/{}/{}", path.author, path.repository, path.branch, path.path);
            
            let html = html! {
                html prefix="og: https://ogp.me/ns#" {
                    head {
                        title { (path.repository) }
                        link rel="canonical" href=(canon_url);
                        meta property="og:image" content=(og_image);
                        meta property="og:title" content=(path.repository);
                        meta property="og:type" content="website";
                        meta property="og:url" content=(canon_url);
                        @if user_agent_string.contains("Telegram") {
                            meta http-equiv="refresh" content=(format!("0;url={}", canon_url));
                        }
                    }
                }
            };

            return HttpResponse::Ok().body(html.into_string());
        }
    }
    
    HttpResponse::TemporaryRedirect().insert_header(("Location", canon_url)).finish()
}

#[get("/{author}/{repository}{path:.*}")]
async fn get_other_pages(req: HttpRequest) -> impl Responder {
    HttpResponse::PermanentRedirect().insert_header(("Location", format!("https://github.com{}", req.uri()))).finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port = std::env::var("PORT")
        .ok()
        .and_then(|port| {port.parse::<u16>().ok()})
        .unwrap_or(8080);

    HttpServer::new(|| {
        App::new()
            .service(get_open_graph)
            .service(get_source_image)
            .service(get_other_pages)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}