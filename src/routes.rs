use std::io::Cursor;

use actix_web::{get, HttpResponse, Responder, HttpRequest, Result, web::{Path, Data, Query}};
use futures_util::StreamExt;
use image::ImageFormat;
use maud::{html, DOCTYPE};
use serde::Deserialize;

use crate::{image_generator, UA_REGEX, Options, utils::{parse_raw_code_uri, QueryLines, clamp_query_lines, parse_raw_gist_code_uri}, errors::RequestError, content::{Content, TextContent, ImageContent, VideoContent, SVGContent, GistContent}};


#[derive(Deserialize)]
pub(crate) struct SrcPath {
    pub(crate) author: String,
    pub(crate) repository: String,
    pub(crate) branch: String,
    pub(crate) path: String
}

#[derive(Deserialize, Debug)]
pub(crate) struct ImgQuery {
    pub(crate) lines: Option<QueryLines>,
    pub(crate) theme: Option<String>,
    pub(crate) font: Option<String>,
    pub(crate) font_size: Option<f32>
}

#[get("/image/{author}/{repository}/{branch}/{path:.*}")]
pub(crate) async fn get_gh_image(path: Path<SrcPath>, query: Query<ImgQuery>, env: Data<Options>) -> Result<impl Responder> {
    let code_uri = parse_raw_code_uri(&path.into_inner())?;
    
    if let Ok(response) = reqwest::get(code_uri.to_string()).await {
        let content_type_string = response.headers().get("Content-Type").and_then(|content_type| { content_type.to_str().ok() }).unwrap_or("");
        if content_type_string.contains("text/plain") {
            let mut query_lines = query.lines.to_owned().unwrap_or(QueryLines::default());
            let mut line: u32 = 0;
            let mut bytes_read: u32 = 0;
            let mut buffer = Vec::new();
            let mut body_stream = response.bytes_stream();

            clamp_query_lines(&mut query_lines);

            let start = query_lines.from - 1;

            while let Some(Ok(chunk)) = body_stream.next().await {
                for byte in chunk {
                    if byte == b'\n' {
                        line += 1;
                    }
                    bytes_read += 1;

                    if bytes_read >= env.MAX_DOWNLOAD_BYTES {
                        break;
                    }
                    
                    if line >= start && line < query_lines.to {
                        buffer.push(byte);
                    }
                    else if line >= query_lines.to {
                        break;
                    }
                }
            }
            if let Ok(src_code) = std::str::from_utf8(&buffer) {
                image_generator::generate_src_image_with_query(src_code, &query).write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png).unwrap();
                return Ok(HttpResponse::Ok()
                    .content_type("image/png")
                    .body(buffer));
            }
        }
        else if content_type_string.contains("image/svg+xml") {
            let mut bytes_read: u32 = 0;
            let mut body_stream = response.bytes_stream();
            let mut buffer = Vec::new();

            while let Some(Ok(chunk)) = body_stream.next().await {
                for byte in chunk {
                    buffer.push(byte);

                    bytes_read += 1;

                    if bytes_read >= env.MAX_DOWNLOAD_BYTES {
                        break;
                    }
                }
            }
            if let Some(image) = image_generator::generate_svg_image(&buffer) {
                return Ok(HttpResponse::Ok()
                    .content_type("image/png")
                    .body(image));
            }
        }
    }

    Ok(HttpResponse::NotFound().body("Unable to fetch code..."))
}

#[get("/{author}/{repository}/blob/{branch}/{path:.*}")]
pub(crate) async fn get_gh_open_graph(req: HttpRequest, path: Path<SrcPath>, query: Query<ImgQuery>, env: Data<Options>) -> Result<impl Responder> {
    let gh_url = format!("https://github.com{}", req.uri());
    let canon_url = format!("{}{}", env.ORIGIN, req.uri());
    
    println!("Graph Visited: {}", canon_url);

    if let Some(user_agent_string) = req.headers().get("User-Agent").and_then(|user_agent| { user_agent.to_str().ok() }) {
        if UA_REGEX.is_match(&user_agent_string.to_lowercase()) {
            
            let code_uri = parse_raw_code_uri(path.as_ref())?;
            let request = reqwest::Client::new().head(code_uri.to_string()).send().await.map_err(RequestError::from)?;
            let content_type_string = request.headers().get("Content-Type").and_then(|content_type| { content_type.to_str().ok() }).unwrap_or("");

            println!("Content-Type: {}", content_type_string);

            let wrapped_injected_elements = 
            if content_type_string.contains("text/plain") {
                let query_lines = query.lines.unwrap_or(QueryLines::default());
                let content = TextContent {
                    path: path.as_ref(),
                    query_string: req.query_string().to_owned(),
                    lines: query_lines,
                    origin: env.ORIGIN.clone()
                };
                Some(content.get_html())
            } 
            else if 
                content_type_string.contains("image/png") ||
                content_type_string.contains("image/jpeg") ||
                content_type_string.contains("image/jpg") ||
                content_type_string.contains("image/gif")
            {
                
                let content = ImageContent {
                    path: path.as_ref(),
                    image_url: code_uri.to_string(),
                    mime: content_type_string.to_owned()
                };
                Some(content.get_html())
            }
            else if content_type_string.contains("image/svg+xml") {
                let content = SVGContent {
                    path: path.as_ref(),
                    origin: env.ORIGIN.clone(),
                };
                Some(content.get_html())
            }
            else if content_type_string.contains("video/mp4") {
                let content = VideoContent {
                    path: path.as_ref(),
                    video_url: code_uri.to_string(),
                    mime: content_type_string.to_owned()
                };
                Some(content.get_html())
            }
            else {
                None
            };

            if let Some(injected_elements) = wrapped_injected_elements {
                let html = html! {
                    (DOCTYPE)
                    html {
                        head {
                            title { (path.repository) }
                            link rel="canonical" href=(canon_url);
                            meta property="og:type" content="website";
                            meta property="og:url" content=(canon_url);
                            meta property="og:site_name" content="GitHub";
    
                            meta property="twitter:domain" content=(env.ORIGIN.replace("http://", "").replace("https://", ""));
                            meta property="twitter:url" content=(canon_url);
                            
                            (injected_elements)
    
                            @if !user_agent_string.contains("Telegram") {
                                meta http-equiv="refresh" content=(format!("0; url={}", gh_url));
                            }
                        }
                        body {
                            "Redirecting to GitHub..."
                        }
                    }
                };
    
                return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html.into_string()));
            }
        }
    }
    
    Ok(HttpResponse::TemporaryRedirect().insert_header(("Location", gh_url)).finish())
}

#[derive(Deserialize)]
pub(crate) struct GistPath {
    pub(crate) author: String,
    pub(crate) id: String
}

#[get("/gist-image/{author}/{id:.*}")]
pub(crate) async fn get_gist_image(path: Path<GistPath>, query: Query<ImgQuery>, env: Data<Options>) -> Result<impl Responder> {
    let code_uri = parse_raw_gist_code_uri(&path.into_inner())?;
    
    if let Ok(response) = reqwest::get(code_uri.to_string()).await {
        let mut query_lines = query.lines.to_owned().unwrap_or(QueryLines::default());
        let mut line: u32 = 0;
        let mut bytes_read: u32 = 0;
        let mut buffer = Vec::new();
        let mut body_stream = response.bytes_stream();

        clamp_query_lines(&mut query_lines);

        let start = query_lines.from - 1;

        while let Some(Ok(chunk)) = body_stream.next().await {
            for byte in chunk {
                if byte == b'\n' {
                    line += 1;
                }
                bytes_read += 1;

                if bytes_read >= env.MAX_DOWNLOAD_BYTES {
                    break;
                }
                
                if line >= start && line < query_lines.to {
                    buffer.push(byte);
                }
                else if line >= query_lines.to {
                    break;
                }
            }
        }
        if let Ok(src_code) = std::str::from_utf8(&buffer) {
            image_generator::generate_src_image_with_query(src_code, &query).write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png).unwrap();
            return Ok(HttpResponse::Ok()
                .content_type("image/png")
                .body(buffer));
        }
    }

    Ok(HttpResponse::NotFound().body("Unable to fetch code..."))
}

#[get("/gist/{author}/{id:.*}")]
pub(crate) async fn get_gist_open_graph(req: HttpRequest, path: Path<GistPath>, query: Query<ImgQuery>, env: Data<Options>) -> Result<impl Responder> {
    let gist_url = format!("https://gist.github.com/{}/{}?{}", path.author, path.id, req.query_string());
    let canon_url = format!("{}{}", env.ORIGIN, req.uri());
    
    println!("Graph Visited: {}", canon_url);

    if let Some(user_agent_string) = req.headers().get("User-Agent").and_then(|user_agent| { user_agent.to_str().ok() }) {
        if UA_REGEX.is_match(&user_agent_string.to_lowercase()) {
            
            let content = GistContent {
                path: path.as_ref(),
                query_string: req.query_string().to_owned(),
                lines: query.lines.unwrap_or(QueryLines::default()),
                origin: env.ORIGIN.clone()
            };

            let html = html! {
                (DOCTYPE)
                html {
                    head {
                        title { (path.id) }
                        link rel="canonical" href=(canon_url);
                        meta property="og:type" content="website";
                        meta property="og:url" content=(canon_url);
                        meta property="og:site_name" content="GitHub Gist";

                        meta property="twitter:domain" content=(env.ORIGIN.replace("http://", "").replace("https://", ""));
                        meta property="twitter:url" content=(canon_url);
                        
                        (content.get_html())

                        @if !user_agent_string.contains("Telegram") {
                            meta http-equiv="refresh" content=(format!("0; url={}", gist_url));
                        }
                    }
                    body {
                        "Redirecting to GitHub..."
                    }
                }
            };

            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html.into_string()));
        }
    }
    
    Ok(HttpResponse::TemporaryRedirect().insert_header(("Location", gist_url)).finish())
}

#[get("/{path:.*}")]
pub(crate) async fn get_other_pages(req: HttpRequest) -> impl Responder {
    HttpResponse::PermanentRedirect().insert_header(("Location", format!("https://github.com{}", req.uri()))).finish()
}