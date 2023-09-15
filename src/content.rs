use maud::{html, PreEscaped};

use crate::{utils::QueryLines, routes::SrcPath};

pub(crate) trait Content {
    fn get_html(&self) -> PreEscaped<String>;
}

pub(crate) struct TextContent<'a> {
    pub(crate) path: &'a SrcPath,
    pub(crate) query_string: String,
    pub(crate) lines: QueryLines,
    pub(crate) origin: String,
}

impl<'a> Content for TextContent<'a> {
    fn get_html(&self) -> PreEscaped<String> {
        let file_name = self.path.path.split("/").last().unwrap_or("<undefined>");
        let og_image = format!("{}/image/{}/{}/{}/{}?{}", self.origin, self.path.author, self.path.repository, self.path.branch, self.path.path, self.query_string);
        let og_description = format!("Lines {}-{} of {} from {}/{}@{}", self.lines.from, self.lines.to, file_name, self.path.author, self.path.repository, self.path.branch);
        html!{
            meta name="description" content=(og_description);
            meta property="og:image" content=(og_image);
            meta property="og:image:type" content="image/png";
            meta property="og:title" content=(self.path.repository);
            meta property="og:description" content=(og_description);

            meta name="twitter:title" content=(self.path.repository);
            meta name="twitter:card" content="summary_large_image";
            meta name="twitter:description" content=(og_description);
            meta name="twitter:image" content=(og_image);
        }
    }
}

pub(crate) struct ImageContent<'a> {
    pub(crate) path: &'a SrcPath,
    pub(crate) image_url: String,
    pub(crate) mime: String,
}

impl<'a> Content for ImageContent<'a> {
    fn get_html(&self) -> PreEscaped<String> {
        let file_name = self.path.path.split("/").last().unwrap_or("<undefined>");
        let og_description = format!("{} from {}/{}@{}", file_name, self.path.author, self.path.repository, self.path.branch);
        html!{
            meta name="description" content=(og_description);
            meta property="og:image" content=(self.image_url);
            meta property="og:image:type" content=(self.mime);
            meta property="og:title" content=(self.path.repository);
            meta property="og:description" content=(og_description);

            meta name="twitter:title" content=(self.path.repository);
            meta name="twitter:card" content="summary_large_image";
            meta name="twitter:description" content=(og_description);
            meta name="twitter:image" content=(self.image_url);
        }
    }
}