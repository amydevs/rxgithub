use actix_web::{http::Uri, Result};

use crate::routes::SrcPath;

pub(crate) fn parse_raw_code_uri(path: &SrcPath) -> Result<Uri> {
    Ok(Uri::builder()
        .scheme("https")
        .authority("raw.githubusercontent.com")
        .path_and_query(format!("/{}/{}/{}/{}", path.author, path.repository, path.branch, path.path))
        .build()?)
}