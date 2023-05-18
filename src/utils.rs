use actix_web::{http::Uri, Result};
use serde::{Deserialize, Deserializer, de};

use crate::{routes::SrcPath, MAX_CODE_LINES};

pub(crate) fn parse_blob_code_uri(path: &SrcPath) -> Result<Uri> {
    Ok(Uri::builder()
        .scheme("https")
        .authority("github.com")
        .path_and_query(format!("/{}/{}/blob/{}/{}", path.author, path.repository, path.branch, path.path))
        .build()?)
}

pub(crate) fn parse_raw_code_uri(path: &SrcPath) -> Result<Uri> {
    Ok(Uri::builder()
        .scheme("https")
        .authority("raw.githubusercontent.com")
        .path_and_query(format!("/{}/{}/{}/{}", path.author, path.repository, path.branch, path.path))
        .build()?)
}

#[derive(Debug)]
pub(crate) struct QueryLines {
    pub(crate) from: usize,
    pub(crate) to: usize
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

                if parts.len() != 2 {
                    return Err(E::custom("invalid format"));
                }

                let from: usize = parts[0]
                    .parse()
                    .map_err(|_| E::custom("invalid 'from' value"))?;
                let to: usize = parts[1]
                    .parse()
                    .map_err(|_| E::custom("invalid 'to' value"))?;

                if from > to {
                    return Err(E::custom("'from' is bigger than 'to'"));
                }

                Ok(QueryLines { from, to })
            }
        }

        deserializer.deserialize_str(QueryLinesVisitor)
    }
}


pub(crate) fn substring_lines_with_max(string: &str, lines: &QueryLines) -> String {
    if (lines.to - lines.from) > MAX_CODE_LINES {
        let revised_lines = QueryLines {
            from: lines.from,
            to: lines.from + MAX_CODE_LINES
        };
        return substring_lines(string, &revised_lines);
    }
    substring_lines(string, lines)
}

pub(crate) fn substring_lines(string: &str, lines: &QueryLines) -> String {
    let mut return_string = String::new();
    for (i, line) in string.lines().enumerate() {
        if lines.from <= i && i <= lines.to {
            return_string += line;
            return_string += "\n";
        }
    }
    
    return_string
}
