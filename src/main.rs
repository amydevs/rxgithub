#[macro_use]
extern crate lazy_static;

use actix_web::{web, App, HttpServer};

use regex::Regex;

use dotenv::dotenv;

mod content;
mod errors;
mod image_generator;
mod routes;
mod utils;

lazy_static! {
    static ref UA_REGEX: Regex = Regex::new(r"bot|facebook|embed|got|firefox/92|firefox/38|curl|wget|go-http|yahoo|generator|whatsapp|preview|link|proxy|vkshare|images|analyzer|index|crawl|spider|python|cfnetwork|node").unwrap();
}

#[derive(Clone)]
struct Options {
    port: u16,
    origin: String,
    max_download_bytes: u32,
    max_code_lines: u32,
}
impl Default for Options {
    fn default() -> Self {
        Options {
            port: 8080,
            origin: "http://localhost:8080".to_string(),
            max_download_bytes: 1024 * 1024 * 50, // 25 MiB
            max_code_lines: 25,
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let default_options = Options::default();

    let options = Options {
        port: std::env::var("PORT")
            .ok()
            .and_then(|port| port.parse::<u16>().ok())
            .unwrap_or(default_options.port),
        origin: std::env::var("ORIGIN").unwrap_or(Options::default().origin),
        max_download_bytes: std::env::var("MAX_DOWNLOAD_BYTES")
            .ok()
            .and_then(|bytes| bytes.parse::<u32>().ok())
            .unwrap_or(default_options.max_download_bytes),
        max_code_lines: std::env::var("MAX_CODE_LINES")
            .ok()
            .and_then(|lines| lines.parse::<u32>().ok())
            .unwrap_or(default_options.max_code_lines),
    };

    let port = options.port;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(options.clone()))
            .app_data(web::Data::new(
                image_generator::TextImageGenerator::default(),
            ))
            .app_data(web::Data::new(image_generator::SvgImageGenerator::default()))
            .service(routes::get_gh_open_graph)
            .service(routes::get_gh_image)
            .service(routes::get_gh_video_embed)
            .service(routes::get_gist_open_graph)
            .service(routes::get_gist_image)
            .service(routes::get_other_pages)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
