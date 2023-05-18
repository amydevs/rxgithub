use std::io::Cursor;

use actix_web::{get, HttpResponse, Responder, HttpRequest, Result, web::{Path, Data, Query}};
use image::ImageFormat;
use maud::{html, DOCTYPE};
use serde::Deserialize;

use crate::{image_generator, UA_REGEX, Options, utils::{parse_raw_code_uri, QueryLines, substring_lines_with_max}, MAX_CODE_LINES};


#[derive(Deserialize)]
pub(crate) struct SrcPath {
    pub(crate) author: String,
    pub(crate) repository: String,
    pub(crate) branch: String,
    pub(crate) path: String
}

#[derive(Deserialize, Debug)]
pub struct SrcQuery {
    lines: Option<QueryLines>
}

#[get("/image/{author}/{repository}/{branch}/{path:.*}")]
pub(crate) async fn get_source_image(path: Path<SrcPath>, query: Query<SrcQuery>) -> Result<impl Responder> {
    let code_uri = parse_raw_code_uri(&path.into_inner())?;

    if let Ok(request) = reqwest::get(code_uri.to_string()).await {
        if let Some(content_type_string) = request.headers().get("Content-Type").and_then(|content_type| { content_type.to_str().ok() }) {
            if !content_type_string.contains("text/plain") {
                return Ok(HttpResponse::TemporaryRedirect().insert_header(("Location", code_uri.to_string())).finish());
            }
            else if let Ok(src_code) = request.text().await {
                let mut buffer = Vec::new();

                let truncated_src_code = query.lines.as_ref().map(|query_lines| substring_lines_with_max(&src_code, query_lines)).unwrap_or(substring_lines_with_max(&src_code, &QueryLines { from: 0, to: MAX_CODE_LINES }));

                image_generator::generate_src_image(&truncated_src_code).write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png).unwrap();
                return Ok(HttpResponse::Ok()
                    .content_type("image/png")
                    .body(buffer));
            }
        }
    }

    Ok(HttpResponse::NotFound().body("Unable to fetch code..."))
}

#[get("/{author}/{repository}/blob/{branch}/{path:.*}")]
pub(crate) async fn get_open_graph(req: HttpRequest, path: Path<SrcPath>, env: Data<Options>) -> Result<impl Responder> {
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

            let og_image = format!("{}/image/{}/{}/{}/{}?{}", env.ORIGIN, path.author, path.repository, path.branch, path.path, req.query_string());
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

#[get("/{path:.*}")]
pub(crate) async fn get_other_pages(req: HttpRequest) -> impl Responder {
    HttpResponse::PermanentRedirect().insert_header(("Location", format!("https://github.com{}", req.uri()))).finish()
}