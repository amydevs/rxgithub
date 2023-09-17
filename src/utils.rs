use actix_web::{http::Uri, Result};
use serde::{de, Deserialize, Deserializer};

use crate::{
    routes::{GistPath, SrcPath},
};

pub(crate) fn parse_blob_code_uri(path: &SrcPath) -> Result<Uri> {
    Ok(Uri::builder()
        .scheme("https")
        .authority("github.com")
        .path_and_query(format!(
            "/{}/{}/blob/{}/{}",
            path.author, path.repository, path.branch, path.path
        ))
        .build()?)
}

pub(crate) fn parse_raw_code_uri(path: &SrcPath) -> Result<Uri> {
    Ok(Uri::builder()
        .scheme("https")
        .authority("raw.githubusercontent.com")
        .path_and_query(format!(
            "/{}/{}/{}/{}",
            path.author, path.repository, path.branch, path.path
        ))
        .build()?)
}

pub(crate) fn parse_raw_gist_code_uri(path: &GistPath) -> Result<Uri> {
    Ok(Uri::builder()
        .scheme("https")
        .authority("gist.githubusercontent.com")
        .path_and_query(format!("/{}/{}/raw", path.author, path.id))
        .build()?)
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct QueryLines {
    pub(crate) from: u32,
    pub(crate) to: Option<u32>,
}

impl Default for QueryLines {
    fn default() -> Self {
        Self {
            from: 1,
            to: None,
        }
    }
}

impl<'de> Deserialize<'de> for QueryLines {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct QueryLinesVisitor;

        impl<'de> de::Visitor<'de> for QueryLinesVisitor {
            type Value = QueryLines;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string in the format 'from-to'")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let parts: Vec<&str> = value.split('-').collect();

                if parts.len() == 0 {
                    return Err(E::custom("invalid format"));
                }

                let from: u32 = parts.get(0).and_then(|from| from.parse().ok()).unwrap_or(1);
                let mut to: Option<u32> = parts.get(1).and_then(|to| to.parse().ok());

                if let Some(to_unwrapped) = to {
                    if to_unwrapped < from {
                        to = None;
                    }
                }

                Ok(QueryLines { from, to })
            }
        }

        deserializer.deserialize_str(QueryLinesVisitor)
    }
}

pub(crate) struct Lines {
    pub(crate) from: u32,
    pub(crate) to: u32
}

pub(crate) fn clamp_query_lines(lines: &QueryLines, max_code_lines: u32) -> Lines {
    if let Some(to) = lines.to {
        if (to - lines.from) < max_code_lines {
            return Lines {
                from: lines.from,
                to
            }
        }
    }
    return Lines { from: lines.from, to: lines.from + max_code_lines - 1 }
}
