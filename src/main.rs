#[macro_use]
extern crate lazy_static;

use actix_web::{App, HttpServer, web::{Data}};

use regex::Regex;

use dotenv::dotenv;

mod routes;
mod utils;
mod image_generator;

lazy_static! {
    static ref UA_REGEX: Regex = Regex::new(r"bot|facebook|embed|got|firefox/92|firefox/38|curl|wget|go-http|yahoo|generator|whatsapp|preview|link|proxy|vkshare|images|analyzer|index|crawl|spider|python|cfnetwork|node").unwrap();
}
static MAX_CODE_LINES: usize = 50;

#[derive(Clone)]
struct Options {
    PORT: u16,
    ORIGIN: String
}
impl Default for Options {
    fn default() -> Self {
        Options {
            PORT: 8080,
            ORIGIN: "http://localhost:8080".to_string()
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let options = Options { 
        PORT: std::env::var("PORT")
            .ok()
            .and_then(|port| {port.parse::<u16>().ok()})
            .unwrap_or(Options::default().PORT),
        ORIGIN: std::env::var("ORIGIN").unwrap_or(Options::default().ORIGIN)
    };

    let port = options.PORT;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(options.clone()))
            .service(routes::get_open_graph)
            .service(routes::get_source_image)
            .service(routes::get_other_pages)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}