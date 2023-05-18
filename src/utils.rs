use actix_web::{App, HttpServer, http::Uri, Result, web::{Data}};

use crate::routes::SrcPath;

pub(crate) fn parse_raw_code_uri(path: &SrcPath) -> Result<Uri> {
    Ok(Uri::builder()
        .scheme("https")
        .authority("raw.githubusercontent.com")
        .path_and_query(format!("/{}/{}/{}/{}", path.author, path.repository, path.branch, path.path))
        .build()?)
}