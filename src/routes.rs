use std::io::Cursor;

use actix_web::{get, App, HttpResponse, HttpServer, Responder, HttpRequest, http::Uri, Result, web::{Path, Data}, Error};
use image::ImageFormat;
use maud::{html, DOCTYPE};

use crate::{image_generator, parse_raw_code_uri, SrcPath, UA_REGEX, Options};

#[get("/image/{author}/{repository}/{branch}/{path:.*}")]
pub async fn get_source_image(_req: HttpRequest, path: Path<SrcPath>) -> Result<impl Responder> {
    let code_uri = parse_raw_code_uri(&path.into_inner())?;
    println!("Image Visited: {}", code_uri);

    if let Ok(request) = reqwest::get(code_uri.to_string()).await {
        if let Some(content_type_string) = request.headers().get("Content-Type").and_then(|content_type| { content_type.to_str().ok() }) {
            if !content_type_string.contains("text/plain") {
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
pub async fn get_open_graph(req: HttpRequest, path: Path<SrcPath>, env: Data<Options>) -> Result<impl Responder> {
    let gh_url = format!("https://github.com{}", req.uri());
    let canon_url = format!("{}{}", env.ORIGIN, req.uri());
    
    println!("Graph Visited: {}", canon_url);

    if let Some(user_agent_string) = req.headers().get("User-Agent").and_then(|user_agent| { user_agent.to_str().ok() }) {
        if UA_REGEX.is_match(&user_agent_string.to_lowercase()) {
            
            let code_uri = parse_raw_code_uri(path.as_ref())?;
            if let Ok(request) = reqwest::Client::new().head(code_uri.to_string()).send().await {
                if let Some(content_type_string) = request.headers().get("Content-Type").and_then(|content_type| { content_type.to_str().ok() }) {
                    if !content_type_string.contains("text/plain") {
                        return Ok(HttpResponse::TemporaryRedirect().insert_header(("Location", gh_url)).finish());
                    }
                }
            }

            let og_image = format!("{}/image/{}/{}/{}/{}", env.ORIGIN, path.author, path.repository, path.branch, path.path);
            let og_description = format!("{}/{}@{}", path.author, path.repository, path.branch);

            let html = html! {
                (DOCTYPE)
                html {
                    head {
                        title { (path.repository) }
                        link rel="canonical" href=(canon_url);
                        meta name="description" content=(og_description);
                        meta property="og:image" content=(og_image);
                        meta property="og:image:type" content="image/png";
                        meta property="og:title" content=(path.repository);
                        meta property="og:description" content=(og_description);
                        meta property="og:type" content="website";
                        meta property="og:url" content=(canon_url);

                        meta name="twitter:card" content="summary_large_image";
                        meta property="twitter:domain" content=(env.ORIGIN.replace("http://", "").replace("https://", ""));
                        meta property="twitter:url" content=(canon_url);
                        meta name="twitter:title" content=(path.repository);
                        meta name="twitter:description" content=(og_description);
                        meta name="twitter:image" content=(og_image);

                        @if !user_agent_string.contains("Telegram") {
                            meta http-equiv="refresh" content=(format!("0; url={}", gh_url));
                        }
                    }
                    body {
                        "Redirecring to GitHub..."
                    }
                }
            };

            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html.into_string()));
        }
    }
    
    Ok(HttpResponse::TemporaryRedirect().insert_header(("Location", gh_url)).finish())
}

#[get("/{author}/{repository}{path:.*}")]
pub async fn get_other_pages(req: HttpRequest) -> impl Responder {
    HttpResponse::PermanentRedirect().insert_header(("Location", format!("https://github.com{}", req.uri()))).finish()
}